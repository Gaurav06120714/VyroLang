# vpm — Vyro Package Manager

## Goal

Cargo-style dependency management for VyroLang: install, publish, update, and remove packages with reliable resolution, version locking, and a registry.

## Commands

```
vpm init                 # create vyro.toml
vpm install <pkg>        # add + resolve + lock
vpm install              # install all from lockfile
vpm update [<pkg>]       # update within semver constraints
vpm remove <pkg>         # remove dependency
vpm publish              # publish to registry
vpm search <query>       # find packages
```

## Manifest — `vyro.toml`

```toml
[package]
name = "my-app"
version = "0.1.0"
author = "Gaurav"
license = "MIT"

[dependencies]
math = "^1.2.0"
http = "~0.4.0"

[dev-dependencies]
test = "^1.0.0"
```

## Lockfile — `vyro.lock`

Pins exact resolved versions + content hashes for reproducible builds. Committed to source control.

## Dependency Resolution

- **SemVer** constraints (`^`, `~`, `=`, ranges).
- Resolver produces a single compatible version per package (MVS-style: minimal version selection) to keep builds stable.
- Conflict reporting with the path that caused it.
- Integrity: each package fetch is verified against its hash.

## Registry

```
Browser/CLI → Registry API → PostgreSQL (metadata) + Object storage (tarballs)
```

| Endpoint | Purpose |
|---|---|
| `GET /packages/:name` | metadata + versions |
| `GET /packages/:name/:version/download` | tarball |
| `POST /packages` | publish (auth required) |
| `GET /search?q=` | search |

### Database Schema (registry)

```sql
CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  owner_id INT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE versions (
  id SERIAL PRIMARY KEY,
  package_id INT REFERENCES packages(id),
  version TEXT NOT NULL,
  manifest JSONB NOT NULL,
  checksum TEXT NOT NULL,
  yanked BOOLEAN DEFAULT false,
  published_at TIMESTAMPTZ DEFAULT now(),
  UNIQUE(package_id, version)
);
```

## Folder Structure (in VyroCoding)

```
vpm/
├── src/
│   ├── manifest/    # parse vyro.toml
│   ├── resolver/    # SemVer resolution
│   ├── lockfile/    # read/write vyro.lock
│   ├── registry/    # client
│   ├── cache/       # local package cache
│   └── main.rs
└── tests/
```

Technology: **Rust** (CLI), **TypeScript** (registry service).

## Security Considerations

- Publish requires an auth token (scoped, revocable).
- Tarball hash verification on every fetch.
- Yank (not delete) for compromised versions.
- Namespace ownership; no typosquatting of reserved names.

## Testing Strategy

- Resolver unit tests (constraints, conflicts).
- Lockfile round-trip + reproducibility tests.
- Registry integration tests (publish→install).

## Estimated Development Time

CLI + resolver + lockfile: ~2 weeks. Registry service: ~2 weeks. ([v0.5.0](../../versions/v0.5.0-vpm-os.md))

## Future Improvements

- Workspaces/monorepos, feature flags, private registries, audit (`vpm audit`) against a vulnerability DB.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Supply-chain attacks | hash verify, scoped tokens, audit, yank |
| Resolution conflicts | clear errors; MVS for stability |
| Registry abuse | rate limits, auth, ownership rules |
