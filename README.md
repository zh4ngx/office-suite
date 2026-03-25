# Office Suite

Local-First Zero agentic office suite. Zero-trust, zero-infrastructure, zero-cloud.

## Core Concepts

- **CRDT Layer**: All state (data + agent memory) lives in Automerge documents that sync peer-to-peer
- **Wasm Sandbox**: Agents run in Wasmtime with strict capability-based security
- **Content-Addressed Storage**: Binary assets stored by SHA-256 hash, referenced by CID

## Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture, data schemas, implementation status

## Workspace Structure

```
office/
├── ARCHITECTURE.md     # Source of truth: design, schemas, status
├── crates/
│   ├── core/           # Shared types, traits, error types
│   ├── crdt/           # Automerge wrapper, document types
│   ├── sync/           # P2P sync (libp2p) + NATS relay
│   ├── sandbox/        # Wasmtime agent runtime, WASI interfaces
│   ├── storage/        # Content-addressed blob store
│   └── apps/
│       ├── calendar/   # Calendar data model, iCal import/export
│       ├── mail/       # Email data model, SMTP/IMAP proxy
│       └── notes/      # Collaborative text editing
├── agents/             # (planned) Wasm agent code
└── examples/           # (planned) Usage examples
```

## MVP Roadmap

1. **Foundation**: core, crdt, storage crates
2. **Calendar**: First app, proves CRDT sync model
3. **Sync Layer**: P2P discovery and document sync
4. **Email**: Mail client with agent assistance
5. **Notes**: Collaborative text editing

## Quick Start

```bash
# Enter dev shell
nix develop

# Build workspace
cargo build

# Run tests
cargo test
```

## Status

Phase 1 (Foundation) - Scaffolded with stubs. See ARCHITECTURE.md for implementation status.
