# ✅ Task Manager REST API

A full-stack task management app built in Rust — REST API backend with a vanilla JS frontend, persisted to SQLite.

---

## Tech stack

| Layer     | Technology                        |
|-----------|-----------------------------------|
| Backend   | Rust, Axum, Tokio (async runtime) |
| Database  | SQLite via sqlx                   |
| Frontend  | Vanilla HTML + CSS + JS           |
| Static    | Served by tower-http ServeDir     |

---

## Project structure

```
TaskManager_RESTAPI/
├── src/
│   ├── main.rs        # Tokio runtime, router setup, server bind
│   ├── models.rs      # Task, CreateTaskInput, UpdateTaskInput structs
│   ├── db.rs          # init_db() — connects to SQLite, runs migration
│   └── handlers.rs    # Async handler fns for each endpoint
├── static/
│   ├── index.html     # App shell
│   ├── style.css      # Styling
│   └── app.js         # fetch() calls to the API
├── migrations/
│   └── 001_tasks.sql  # CREATE TABLE tasks
├── .env               # DATABASE_URL=sqlite://tasks.db
└── Cargo.toml
```

---

## API endpoints

| Method   | Route        | Description          | Body                        |
|----------|--------------|----------------------|-----------------------------|
| `GET`    | `/tasks`     | Get all tasks        | —                           |
| `POST`   | `/tasks`     | Create a task        | `{ "title": "..." }`        |
| `GET`    | `/tasks/:id` | Get one task by ID   | —                           |
| `PUT`    | `/tasks/:id` | Update title / done  | `{ "title": "...", "done": true }` |
| `DELETE` | `/tasks/:id` | Delete a task        | —                           |

---

## Running locally

### Prerequisites
- Rust installed (`rustup`)
- No other setup needed — SQLite is embedded

### Steps

```bash
git clone https://github.com/ReetSolanki/TaskManager_RESTAPI
cd TaskManager_RESTAPI
```

Create a `.env` file:
```
DATABASE_URL=sqlite://tasks.db
```

Run:
```bash
cargo run
```

Open `http://localhost:3000` in your browser.

---

## Testing the API with curl

```bash
# Get all tasks
curl http://localhost:3000/tasks

# Create a task
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy milk"}'

# Get one task
curl http://localhost:3000/tasks/1

# Update a task
curl -X PUT http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy milk", "done": true}'

# Delete a task
curl -X DELETE http://localhost:3000/tasks/1
```

---

## Frontend

The UI is served directly from Axum using `tower-http`'s `ServeDir` — no separate server needed.

Features:
- Add tasks with Enter key or Add button
- Check off tasks to mark as done
- Inline edit — click ✏️ to edit title in place
- Delete tasks with ✕

---

## What I learned building this

- Async Rust with Tokio — `async fn`, `.await`, `#[tokio::main]`
- Axum routing, extractors (`State`, `Json`, `Path`)
- sqlx for compile-time checked async SQL queries
- Serving a static frontend from the same Axum server with `tower-http`
- Separating input/output types (`CreateTaskInput` vs `Task`)

---

## Next

Deploying to Railway with a live demo link.
