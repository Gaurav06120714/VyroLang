//! VyroLang Compiler API — a zero-dependency HTTP server.
//!
//! This is the "Compiler API" box from the architecture diagram: the browser
//! (VyroIDE) posts source here; we compile it with VyroCompiler, run the
//! bytecode on a sandboxed VyroVM (instruction + time limits, captured output,
//! request-fed stdin), and return the result as JSON.
//!
//!   GET  /            -> the VyroIDE (embedded single-page app)
//!   GET  /health      -> "ok"
//!   POST /api/run     -> { code, stdin } => { ok, status, stdout, error, timeMs }
//!   POST /api/compile -> { code }        => { ok, status, error }

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

use crate::compile_source;
use crate::vm::Vm;

const INDEX_HTML: &str = include_str!("../web/index.html");

const INSTR_BUDGET: u64 = 50_000_000;
const TIME_LIMIT_MS: u64 = 5_000;

pub fn serve(port: u16) -> std::io::Result<()> {
    // Bind 127.0.0.1 by default; containers set VYRO_HOST=0.0.0.0 to expose the port.
    let host = std::env::var("VYRO_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr)?;
    println!("VyroLang Compiler API + VyroIDE listening on http://{}", addr);
    println!("Open it in your browser and start coding.");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    let _ = handle(s);
                });
            }
            Err(e) => eprintln!("connection error: {}", e),
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.try_clone()?;
    let mut reader = BufReader::new(peer);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    let mut content_length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 {
            break;
        }
        if header == "\r\n" || header == "\n" {
            break;
        }
        let lower = header.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            content_length = v.trim().parse().unwrap_or(0);
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }
    let body = String::from_utf8_lossy(&body).to_string();

    let (status, ctype, payload) = route(method, path, &body);
    let resp = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        status,
        ctype,
        payload.len()
    );
    stream.write_all(resp.as_bytes())?;
    stream.write_all(payload.as_bytes())?;
    stream.flush()
}

fn route(method: &str, path: &str, body: &str) -> (&'static str, &'static str, String) {
    match (method, path) {
        ("GET", "/") => ("200 OK", "text/html; charset=utf-8", INDEX_HTML.to_string()),
        ("GET", "/health") => ("200 OK", "text/plain", "ok".to_string()),
        ("OPTIONS", _) => ("204 No Content", "text/plain", String::new()),
        ("POST", "/api/run") => ("200 OK", "application/json", run_json(body)),
        ("POST", "/api/compile") => ("200 OK", "application/json", compile_json(body)),
        _ => ("404 Not Found", "application/json", r#"{"error":"not found"}"#.to_string()),
    }
}

fn run_json(body: &str) -> String {
    let fields = parse_json_object(body);
    let code = fields.get("code").cloned().unwrap_or_default();
    let stdin = fields.get("stdin").cloned().unwrap_or_default();

    let main = match compile_source(&code) {
        Ok(m) => m,
        Err(diag) => {
            return obj(&[
                ("ok", "false"),
                ("status", &q("compile_error")),
                ("stdout", &q("")),
                ("error", &q(&diag)),
            ]);
        }
    };

    let started = Instant::now();
    let mut vm = Vm::sandboxed(&stdin, INSTR_BUDGET, TIME_LIMIT_MS);
    let (stdout, err) = vm.run_collect(main);
    let ms = started.elapsed().as_millis();

    match err {
        None => obj(&[
            ("ok", "true"),
            ("status", &q("ok")),
            ("stdout", &q(&stdout)),
            ("error", "null"),
            ("timeMs", &ms.to_string()),
        ]),
        Some(e) => {
            let status = if e.contains("timed out") || e.contains("instruction budget") {
                "time_limit_exceeded"
            } else {
                "runtime_error"
            };
            obj(&[
                ("ok", "false"),
                ("status", &q(status)),
                ("stdout", &q(&stdout)),
                ("error", &q(&e)),
                ("timeMs", &ms.to_string()),
            ])
        }
    }
}

fn compile_json(body: &str) -> String {
    let fields = parse_json_object(body);
    let code = fields.get("code").cloned().unwrap_or_default();
    match compile_source(&code) {
        Ok(_) => obj(&[("ok", "true"), ("status", &q("compiled")), ("error", "null")]),
        Err(diag) => obj(&[("ok", "false"), ("status", &q("compile_error")), ("error", &q(&diag))]),
    }
}

// ---- tiny JSON helpers (no external deps) ----

/// Build a JSON object from already-encoded (key, raw-value) pairs.
fn obj(pairs: &[(&str, &str)]) -> String {
    let inner: Vec<String> = pairs.iter().map(|(k, v)| format!("\"{}\":{}", k, v)).collect();
    format!("{{{}}}", inner.join(","))
}

/// Quote + escape a string into a JSON string literal.
fn q(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// Parse the top-level string fields of a JSON object. Robust to strings that
/// themselves contain braces, quotes, or other keys.
fn parse_json_object(s: &str) -> HashMap<String, String> {
    let cs: Vec<char> = s.chars().collect();
    let n = cs.len();
    let mut i = 0;
    let mut map = HashMap::new();
    while i < n && cs[i] != '{' {
        i += 1;
    }
    if i < n {
        i += 1;
    }
    loop {
        while i < n && (cs[i].is_whitespace() || cs[i] == ',') {
            i += 1;
        }
        if i >= n || cs[i] == '}' {
            break;
        }
        if cs[i] != '"' {
            break;
        }
        let key = match parse_json_string(&cs, &mut i) {
            Some(k) => k,
            None => break,
        };
        while i < n && (cs[i].is_whitespace() || cs[i] == ':') {
            i += 1;
        }
        if i >= n {
            break;
        }
        if cs[i] == '"' {
            if let Some(v) = parse_json_string(&cs, &mut i) {
                map.insert(key, v);
            } else {
                break;
            }
        } else {
            // skip a non-string value (number, bool, null, object, array)
            let mut depth = 0i32;
            while i < n {
                match cs[i] {
                    '{' | '[' => depth += 1,
                    '}' | ']' => {
                        if depth == 0 {
                            break;
                        }
                        depth -= 1;
                    }
                    ',' if depth == 0 => break,
                    _ => {}
                }
                i += 1;
            }
        }
    }
    map
}

fn parse_json_string(cs: &[char], i: &mut usize) -> Option<String> {
    if *i >= cs.len() || cs[*i] != '"' {
        return None;
    }
    *i += 1;
    let mut out = String::new();
    while *i < cs.len() {
        let c = cs[*i];
        if c == '\\' {
            *i += 1;
            if *i >= cs.len() {
                break;
            }
            match cs[*i] {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                'r' => out.push('\r'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'u' => {
                    let end = (*i + 5).min(cs.len());
                    let hex: String = cs[*i + 1..end].iter().collect();
                    if let Ok(v) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(v) {
                            out.push(ch);
                        }
                    }
                    *i += 4;
                }
                other => out.push(other),
            }
        } else if c == '"' {
            *i += 1;
            return Some(out);
        } else {
            out.push(c);
        }
        *i += 1;
    }
    None
}
