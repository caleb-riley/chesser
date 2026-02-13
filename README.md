# Chesser

Chesser is a turn-based chess game written in Rust. It supports standard and custom pieces, with a server backend for persistence. Multiplayer functionality is planned for the future.

---

## Project Structure

This is a Rust workspace with three main crates:

chesser/  
├── Cargo.toml # workspace root  
├── chesser_core/ # shared game logic (board, pieces, move validation)  
├── chesser_server/ # backend server (Axum, SQLx, SQLite)  
└── chesser_client/ # frontend client (Bevy + Bevy Egui)


- **chesser_core** – Contains the authoritative game engine: board, pieces, move resolution. No networking or GUI dependencies.  
- **chesser_server** – Axum HTTP server storing game state in SQLite. Validates all moves.  
- **chesser_client** – Bevy + Bevy Egui frontend. Displays the board and allows playing locally against game logic.

---

## Features (Current)

- Server-authoritative game logic
- Persistent game state with SQLite
- Support for custom piece types
- Move captures vector
- Local game play (no multiplayer yet)
- Basic GUI with Bevy Egui
- CORS-enabled server for future web client access

---

## Tech Stack

- Rust – core language
- Bevy + Bevy Egui – frontend graphics and GUI
- Axum + Tower-HTTP – backend server and middleware
- SQLx + SQLite – database for storing game state and moves
- Tokio – async runtime

---

## Running the Project

### Build the Workspace

cargo build


### Run the Server

cargo run -p chesser_server


- Listens on `0.0.0.0:3000` by default  
- Uses SQLite database at `../database/data.db`  

### Run the Client

cargo run -p chesser_client


- Currently plays a local game against the game logic  
- Multiplayer support will be added in the future

---

## Game Data Storage

- Each move currently records:
  - Destination square
  - Move type (`Passive`, `Capture`, `Promotion` planned)
  - Captured squares
- Server maintains authoritative state and persists it in SQLite.  
- Full board state can be serialized for saving/loading.

---

## Future Work

- Multiplayer support over TCP/WebSocket  
- Full move history and replay  
- Undo/redo support  
- Promotion and custom piece mechanics fully implemented  
- Custom piece editor
