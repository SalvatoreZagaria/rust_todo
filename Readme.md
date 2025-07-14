# Tasks

A minimal to-do manager in Rust, with two modes:

- **CLI** (`tasks-cli`) — command-line interface
- **Server** (`tasks-server`) — REST API over HTTP

All core task-management logic lives in `src/lib.rs`. The two binaries are thin wrappers:

- **`src/bin/cli.rs`** — parses commands with Clap and reads/writes `.tasks.json`.
- **`src/bin/server.rs`** — exposes the same functionality via Warp + Tokio on port 3030.

---

## Prerequisites

- Rust toolchain (1.70+), with `cargo`
- Internet for fetching crates on first build

---

## Build

From the project root:

```bash
# compile both binaries
cargo build --release

# or compile just one
cargo build --bin tasks-cli
cargo build --bin tasks-server
```

## Run as CLI
Reads/writes `.tasks.json` in the current directory.

### Add a task
`cargo run --bin tasks-cli -- add "Buy groceries"`

### List tasks
`cargo run --bin tasks-cli -- list`

### Mark done
`cargo run --bin tasks-cli -- done <TASK_ID>`

### Remove a task
`cargo run --bin tasks-cli -- remove <TASK_ID>`

### Clear completed tasks
`cargo run --bin tasks-cli -- clear`


## Run as Server
Starts an HTTP server on 127.0.0.1:8080 and uses the same `.tasks.json` file.

`cargo run --bin tasks-server`
### Endpoints
- **GET** `/tasks`
- **POST** `/tasks` `{ "description": "..." }`
- **PUT** `/tasks/{id}/done`
- **DELETE** `/tasks/{id}/delete`
- **DELETE** `/tasks/done`