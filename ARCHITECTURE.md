# Architecture: Local-First Zero Agentic Office Suite

> **Source of truth for this project.** Keep this document in sync with code.

## Current Status

**Phase 1: Foundation** - Scaffolded, minimal implementation.

| Component | Status |
|-----------|--------|
| Workspace structure (8 crates) | ✅ |
| NixOS dev shell | ✅ |
| Core types | ✅ Stub |
| CRDT operations | ❌ Stub only |
| P2P/NATS sync | ❌ Stub only |
| Wasm agents | ❌ Stub only |
| iCal parsing | ❌ Empty |

---

## Dev Environment

```bash
nix develop          # Enter dev shell
cargo check          # Check compilation
cargo test           # Run tests
cargo watch -x check # Auto-rebuild on changes
```

Requirements: Nix with flakes. direnv optional.

---

## Core Pillars

### 1. CRDT Data Layer (`crates/crdt`)
All application state and agent memory lives in **Automerge documents** that sync peer-to-peer. No central server required.

```
┌─────────────────────────────────────────────────────────┐
│                    Automerge CRDT Layer                  │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────┐ │
│  │ Calendar│  │  Email  │  │  Notes  │  │Agent Memory │ │
│  │  Doc    │  │  Doc    │  │  Doc    │  │   Doc       │ │
│  └─────────┘  └─────────┘  └─────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 2. Wasm Agent Sandbox (`crates/sandbox`)
All agent code runs in **Wasmtime** with capability-based security. Agents cannot access network, filesystem, or system resources directly—they go through our WASI interfaces.

### 3. P2P Sync Layer (`crates/sync`)
Devices discover each other and sync CRDT state directly. Fallback via **NATS** relay.

### 4. Content-Addressed Storage (`crates/storage`)
Binary assets stored as **SHA-256 → blob** mappings. CRDT documents reference by CID.

---

## Component Diagram

```
┌──────────────────────────────────────────────────────────────────────┐
│                           User Interface                              │
│                    (CLI → TUI → GUI over time)                        │
└─────────────────────────────────┬────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────┐
│                        Agent Orchestrator                             │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  Primary Agent (Wasm)                                          │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │  │
│  │  │ Calendar │  │  Mail    │  │  Notes   │  │  Coordinator │   │  │
│  │  │ Agent    │  │  Agent   │  │  Agent   │  │  Agent       │   │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────────┘   │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                                  │                                    │
│                        WASI Capability Interfaces                     │
└──────────────────────────────────┼────────────────────────────────────┘
                                   │
         ┌─────────────────────────┼─────────────────────────┐
         │                         │                         │
         ▼                         ▼                         ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  wasi:crdt/*    │     │ wasi:storage/*  │     │ wasi:network/*  │
└────────┬────────┘     └──────┬──────────┘     └──────┬──────────┘
         │                     │                       │
         ▼                     ▼                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Sync Layer                                   │
│  ┌────────────────┐                         ┌──────────────────┐    │
│  │  P2P (libp2p)  │◄────── NATS Relay ─────►│  Sync Server     │    │
│  │  Direct Sync   │      (fallback)         │  (optional)      │    │
│  └────────────────┘                         └──────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

---

## WASI Capability Interfaces

### `wasi:crdt` - CRDT Operations
```wit
interface crdt {
  resource document {
    constructor(id: string);
    get(path: string) -> option<value>;
    set(path: string, value: value) -> result<_, error>;
    subscribe() -> stream<change>;
  }
  get-document(id: string) -> result<document, error>;
  list-documents(filter: option<string>) -> list<string>;
}
```

### `wasi:storage` - Content-Addressed Blobs
```wit
interface storage {
  store(data: list<u8>) -> string;  // returns CID
  fetch(cid: string) -> result<list<u8>, error>;
  exists(cid: string) -> bool;
}
```

### `wasi:network` - External Protocol Proxy
```wit
interface network {
  send-email(msg: email-message) -> result<_, error>;
  fetch-emails(since: option<datetime>) -> list<email-message>;
}
```

### `wasi:agent` - Inter-Agent Communication
```wit
interface agent {
  spawn(agent-type: string, config: option<config>) -> agent-handle;
  send(handle: agent-handle, message: value) -> result<_, error>;
  receive(handle: agent-handle) -> stream<value>;
}
```

---

## Target Data Schemas

> **Note:** These are target schemas. Current code has minimal stubs. See implementation status at end.

### Document Topology

```
workspace/
├── calendar/
│   ├── events.amrg       # Calendar events
│   └── metadata.amrg     # Calendars, subscriptions
├── mail/
│   ├── inbox.amrg        # Mail metadata, flags
│   ├── drafts.amrg       # Compose-in-progress
│   └── threads.amrg      # Conversation threading
├── notes/
│   └── notes.amrg        # All notes
└── agents/
    ├── primary/memory.amrg
    ├── calendar/memory.amrg
    └── mail/memory.amrg
```

### Calendar Event

```json
{
  "events": {
    "<event-id>": {
      "id": "evt_abc123",
      "title": "Lunch with Alice",
      "description": "Discuss Q2 planning",
      "start": "2024-03-25T12:00:00-07:00",
      "end": "2024-03-25T13:00:00-07:00",
      "timezone": "America/Los_Angeles",
      "location": "Cafe Blue",
      "attendees": ["alice@example.com"],
      "recurrence": null,
      "reminders": [15, 60],
      "status": "confirmed",
      "ical_uid": "evt_abc123@local",
      "created_at": "2024-03-24T10:00:00Z",
      "modified_at": "2024-03-24T10:00:00Z",
      "modified_by": "device_laptop_001"
    }
  },
  "_meta": { "version": 1, "created": "2024-01-01T00:00:00Z" }
}
```

### Email Message

```json
{
  "messages": {
    "<msg-id>": {
      "id": "msg_xyz789",
      "thread_id": "thread_abc",
      "message_id": "<abc123@mail.example.com>",
      "from": "bob@example.com",
      "to": ["user@local"],
      "cc": [],
      "subject": "Re: Project Update",
      "preview": "Thanks for the update...",
      "date": "2024-03-24T09:30:00Z",
      "flags": ["seen", "answered"],
      "labels": ["work", "important"],
      "attachment_cids": ["bafyrei..."],
      "folder": "inbox"
    }
  },
  "threads": {
    "<thread-id>": {
      "id": "thread_abc",
      "subject": "Project Update",
      "message_ids": ["msg_1", "msg_2", "msg_xyz789"],
      "participants": ["bob@example.com", "user@local"],
      "last_message_at": "2024-03-24T09:30:00Z"
    }
  }
}
```

**Note:** Email body and attachments stored in content-addressed storage, referenced by CID.

### Note

```json
{
  "notes": {
    "<note-id>": {
      "id": "note_001",
      "title": "Meeting Notes - March 24",
      "content": "# Meeting Notes\n\n- Discussed...",
      "format": "markdown",
      "tags": ["meeting", "work"],
      "created_at": "2024-03-24T14:00:00Z",
      "modified_at": "2024-03-24T14:30:00Z"
    }
  }
}
```

**CRDT Text:** The `content` field uses Automerge's `Text` type for character-level collaborative editing.

### Agent Memory

The core insight: agent memory is structured data that syncs like any other document.

```json
{
  "identity": { "name": "Primary Assistant", "version": "1.0.0" },
  "user_profile": {
    "preferences": { "timezone": "America/Los_Angeles" },
    "patterns": { "common_contacts": ["alice@example.com"] }
  },
  "episodes": {
    "<episode-id>": {
      "timestamp": "2024-03-24T10:00:00Z",
      "type": "calendar_create",
      "summary": "User created lunch event with Alice",
      "entities": { "people": ["Alice"], "locations": ["Cafe Blue"] },
      "outcome": "success"
    }
  },
  "embeddings_index": {
    "chunk_001": "bafyrei...embedding"
  }
}
```

---

## Data Flow Example

```
User: "Schedule lunch with Alice tomorrow at noon"
                │
                ▼
        ┌──────────────┐
        │ Primary Agent│  parses intent, extracts entities
        └──────┬───────┘
               │
               ▼
        ┌──────────────┐
        │Calendar Agent│  validates, creates event struct
        └──────┬───────┘
               │ calls wasi:crdt/set()
               ▼
        ┌──────────────┐
        │  CRDT Layer  │  applies change to Automerge doc
        └──────┬───────┘
               │ emits change
               ▼
        ┌──────────────┐
        │  Sync Layer  │  propagates to other devices
        └──────────────┘
```

---

## Security Model

### Zero Trust Principles
1. **Agents are untrusted** - run in Wasm sandbox with no direct system access
2. **Capabilities are explicit** - each agent gets only the interfaces it needs
3. **Data is local** - nothing leaves devices without explicit action
4. **Sync is authenticated** - only authorized devices can sync

### Capability Grants

| Agent Type | wasi:crdt | wasi:storage | wasi:network | wasi:agent |
|------------|-----------|--------------|--------------|------------|
| Primary    | ✓         | ✓            | ✓            | ✓          |
| Calendar   | ✓ (cal/*) | ✓            | ✗            | ✓          |
| Mail       | ✓ (mail/*)| ✓            | ✓            | ✓          |
| Notes      | ✓ (notes/*)| ✓           | ✗            | ✗          |

---

## Sync Topology

### Primary: Direct P2P (libp2p)
```
    ┌──────────┐                    ┌──────────┐
    │  Laptop  │◄──── libp2p ──────►│  Phone   │
    └──────────┘                    └──────────┘
         │                               │
         │         ┌──────────┐          │
         └────────►│ Desktop  │◄─────────┘
                   └──────────┘
```

### Fallback: NATS Relay
```
    ┌──────────┐      ┌─────────┐      ┌──────────┐
    │  Laptop  │◄────►│  NATS   │◄────►│  Phone   │
    └──────────┘      │ Relay   │      └──────────┘
                      └─────────┘
```

NATS relay is **dumb** - passes messages only. Encryption and CRDT merge happen client-side.

---

## CRDT Conflict Resolution

### Automatic (Automerge)
- **Concurrent field updates**: Last-writer-wins with actor ID + counter
- **Array insertions**: Preserved order via CRDT list semantics
- **Text edits**: Character-level merge (like Google Docs)

### Semantic (Application-Level)

| Conflict Type | Resolution Strategy |
|---------------|---------------------|
| Event time overlap | Surface to user, suggest alternatives |
| Deleted vs modified | Keep both, flag for review |
| Recurrence exception conflict | Prefer more specific rule |
| Mail flag conflict | Union (both can be true) |

---

## Implementation Phases

### Phase 1: Foundation *(in progress)*
- [x] `crates/core` - shared types (scaffolded)
- [x] `crates/crdt` - Automerge wrapper (scaffolded)
- [x] `crates/storage` - blob store (scaffolded)
- [ ] Full CRDT document operations
- [ ] SHA-256 CID computation

### Phase 2: Calendar MVP
- [x] `crates/apps/calendar` - data model (scaffolded, minimal)
- [ ] iCal import/export
- [ ] Full event schema (attendees, recurrence, reminders)
- [ ] CLI interface

### Phase 3: Sync Layer
- [x] `crates/sync` - libp2p + NATS (scaffolded)
- [ ] P2P discovery and sync protocol
- [ ] NATS relay implementation

### Phase 4: Email
- [x] `crates/apps/mail` - data model (scaffolded, minimal)
- [ ] Full message schema
- [ ] SMTP/IMAP proxy

### Phase 5: Notes
- [x] `crates/apps/notes` - data model (scaffolded, minimal)
- [ ] Collaborative text editing
- [ ] TUI/GUI interface

### Phase 6: Wasm Agents
- [x] `crates/sandbox` - Wasmtime runtime (scaffolded)
- [ ] Custom WASI interfaces
- [ ] Capability enforcement
- [ ] Agent loading/execution

---

## Open Questions

1. **Conflict resolution UX** - How to surface semantic conflicts (e.g., two events at same time)?
2. **Agent memory compression** - Strategies: summarization, tiered storage, time-windowing?
3. **Large blob sync** - Sync CID reference first, lazy-load blob on demand?
4. **Multi-user collaboration** - Extend single-user-multi-device to sharing?

---

## Implementation Status

| Schema | Target Fields | Current Code |
|--------|---------------|--------------|
| **Calendar Event** | id, title, description, start, end, timezone, location, attendees, recurrence, reminders, status, ical_uid | id, title, description, start, end, location, created, modified |
| **Email Message** | id, thread_id, message_id, from, to, cc, subject, preview, date, flags, labels, attachment_cids, folder | id, subject, from, to, body, date |
| **Email Thread** | id, subject, message_ids, participants, last_message_at | id, messages, subject |
| **Note** | id, title, content, format, tags, created_at, modified_at | id, title, content, created, modified |
| **Agent Memory** | identity, user_profile, episodes, embeddings_index | *not implemented* |

**Missing from all:**
- CRDT integration (documents don't persist to Automerge yet)
- `_meta` version tracking
- `modified_by` device tracking
