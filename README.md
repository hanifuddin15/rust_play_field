# Axum + Dioxus Fullstack Cargo Workspace

A production-ready fullstack Rust workspace combining an **Axum** asynchronous backend with a reactive **Dioxus** frontend, connected through a shared data model crate for end-to-end type safety.

---

## 📁 Workspace Architecture

```text
.
├── Cargo.toml                  # Cargo workspace manifest
├── crates/
│   ├── shared/                 # Shared data contracts (DTOs, models, envelopes)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs          # TodoItem, CreateTodoRequest, ApiResponse
│   ├── backend/                # Axum asynchronous HTTP server
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs         # Server entry point, CORS, routing & static fallback
│   │       ├── handlers.rs     # REST handlers (CRUD operations)
│   │       └── state.rs        # In-memory thread-safe state (RwLock)
│   └── frontend/               # Dioxus Web UI application
│       ├── Cargo.toml
│       ├── Dioxus.toml         # Dioxus CLI configuration & dev proxy
│       ├── index.html          # HTML shell template
│       ├── assets/
│       │   └── style.css       # Clean responsive styles
│       └── src/
│           ├── main.rs         # Root Dioxus component with signals & reactivity
│           ├── api.rs          # Gloo-net async HTTP client
│           └── components/
│               ├── mod.rs
│               ├── header.rs   # Header and badges
│               ├── item_card.rs# Task card with toggle and delete
│               └── item_form.rs# Task creation form
└── scripts/
    ├── dev-backend.sh          # Start Axum backend
    ├── dev-frontend.sh         # Start Dioxus frontend dev server with hot-reload
    └── build.sh                # Production build pipeline
```

---

## 🛠️ Prerequisites

To build and run this workspace, you'll need:

1. **Rust & Cargo**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WebAssembly Target** (for Dioxus web):
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Dioxus CLI (`dx`)**:
   ```bash
   # Install with cargo-binstall (fast binary install)
   cargo install cargo-binstall
   cargo binstall dioxus-cli

   # Or standard compile from source
   cargo install dioxus-cli
   ```

---

## 🚀 Getting Started

### 1. Run the Axum Backend

Start the backend on `http://127.0.0.1:3000`:

```bash
cargo run -p backend
# or via script:
./scripts/dev-backend.sh
```

Available REST Endpoints:
- `GET    /api/health` — Health check
- `GET    /api/todos` — List all tasks
- `POST   /api/todos` — Create a new task
- `GET    /api/todos/:id` — Retrieve a single task
- `PUT    /api/todos/:id` — Update or toggle completion
- `DELETE /api/todos/:id` — Delete a task

---

### 2. Run the Dioxus Frontend

In a separate terminal window:

```bash
cd crates/frontend
dx serve
# or via script from workspace root:
./scripts/dev-frontend.sh
```

Open [http://localhost:8080](http://localhost:8080) in your browser. Dioxus provides hot-reloading for instant feedback when modifying components or styles.

---

## 📦 Production Build

To build a standalone production release:

1. Build the Dioxus web assets to `crates/frontend/dist`:
   ```bash
   cd crates/frontend
   dx build --release
   cd ../..
   ```

2. Build the backend binary:
   ```bash
   cargo build --release -p backend
   ```

3. Run the binary:
   ```bash
   ./target/release/backend
   ```
   The Axum backend automatically serves both the `/api` endpoints and the compiled frontend static files from `crates/frontend/dist` on `http://127.0.0.1:3000`.

---

## 💡 Key Design Highlights

- **Single Source of Truth**: The `shared` crate defines JSON-serializable types (`TodoItem`, `CreateTodoRequest`, `ApiResponse<T>`) used directly by both server and client without code duplication.
- **CORS Configured**: Cross-Origin Resource Sharing is enabled out of the box so `dx serve` on port `8080` can seamlessly communicate with Axum on port `3000`.
- **Dioxus 0.6 Signals**: Uses modern reactive primitives (`use_signal`, `use_effect`, `spawn`) for efficient and expressive UI updates.

