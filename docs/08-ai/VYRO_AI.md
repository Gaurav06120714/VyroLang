# VyroAI — AI Assistant

## Goal

Weave AI into the authoring loop: completion, review, bug detection, optimization hints, and documentation — the Copilot of the Vyro ecosystem.

## Capabilities

| Capability | Description |
|---|---|
| Code completion | inline + multi-line suggestions, context-aware |
| Code review | flags smells, risky patterns, style issues on diffs |
| Bug detection | static + AI heuristics over the AST and source |
| Optimization suggestions | hints from compiler IR + AI (e.g., hoist loop invariants) |
| Documentation generation | docstrings, README sections, usage examples |

## Architecture

```
VyroIDE ──► AI Gateway ──► Model Provider (Claude — latest model)
                │
                ├─ Context builder: open files, symbols, compiler diagnostics, AST slices
                ├─ Retrieval: workspace + package docs (RAG)
                └─ Guardrails: rate limits, PII redaction, output validation
```

> Default to the latest, most capable Claude model for code tasks. The gateway abstracts the provider so models can be swapped.

## Context Building

VyroAI is grounded in real program structure, not just text:

- Open buffers + cursor region.
- Symbol table and types from the compiler/LSP.
- Current diagnostics (errors/warnings).
- Relevant package docs via retrieval.

This makes suggestions type-aware and project-aware.

## API Surface

| Endpoint | Purpose |
|---|---|
| `POST /ai/complete` | code completion (streaming/SSE) |
| `POST /ai/review` | review a diff or file |
| `POST /ai/explain` | explain selected code |
| `POST /ai/docs` | generate documentation |
| `POST /ai/fix` | suggest a fix for a diagnostic |

## Security & Privacy

- Per-user/workspace consent for sending code to the model.
- PII/secret redaction before egress.
- Rate limiting and per-account quotas.
- No training on private code; logs scrubbed.
- Output is suggestion-only; never auto-applied without user action.

## Testing Strategy

- Golden prompts → expected behavior categories.
- Regression suite for completion quality on sample repos.
- Guardrail tests (redaction, rate limits).
- Latency budgets enforced in CI.

## Estimated Development Time

Gateway + completion: ~2 weeks. Review/docs/fix + RAG: ~2–3 weeks. ([v0.8.0](../../versions/v0.8.0-ai.md))

## Future Improvements

- Agentic refactors across files, test generation, PR summaries, and a local/offline model option.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Leaking private code | consent + redaction + no-train policy |
| Hallucinated APIs | ground in symbol table; validate against compiler |
| Cost overruns | quotas, caching, smaller models for cheap tasks |
