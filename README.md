# rusty-gbrl / MeshForge

Rust CNC control library for GRBL-HAL and the MeshForge Tauri desktop app.

## Library (rusty-gbrl)

- **`machines::grbl`** — GRBL-HAL communication (parser, commands, state, port, poller, streamer, motion). Use the `serial` feature for hardware: `cargo build --features serial`.
- **`machines::session`** — Black box recorder (probe + status logging to JSONL).
- **`machines::profiles`** — Per-machine config (work area, steps/mm, tool library).

## Tauri desktop app (MeshForge)

The UI is a Tauri 2 app in `src-tauri/` with an **Angular 19** frontend in `angular-ui/`. (The original vanilla HTML/JS UI is still in `ui/` for reference; Tauri is configured to use Angular.)

### Run the app

1. **Install dependencies**
   - **Rust:** [rustup](https://rustup.rs)
   - **Node.js:** for `npm run tauri dev` and the Angular app
   - **Linux (WSL2 or native):** Tauri needs the GTK/WebKit stack; serial needs pkg-config and libudev. From the repo root run:
     ```bash
     chmod +x scripts/setup-linux-deps.sh && ./scripts/setup-linux-deps.sh
     ```
     Or install manually:
     ```bash
     sudo apt install -y libwebkit2gtk-4.1-dev build-essential libxdo-dev libssl-dev \
       libayatana-appindicator3-dev librsvg2-dev pkg-config libudev-dev
     ```
   - **Windows:** no extra deps for serial; ensure a driver for your USB–serial adapter

2. **Install frontend tooling (once)**  
   Node.js **18.19+** is required for the Angular app. Then:
   ```bash
   npm install
   cd angular-ui && npm install && cd ..
   ```

3. **Development**
   ```bash
   npm run dev
   ```
   Starts the Angular dev server (port 4200) and opens the Tauri window. Use **Refresh ports** and (in mock mode) **Refresh status**.

   **Test without hardware (mock API):**
   ```bash
   npm run dev:mock
   ```
   Runs with `MESHFORGE_MOCK=1`: fake serial ports and mock machine status so you can exercise the UI with no machine connected.

4. **Production build**
   ```bash
   npm run build
   ```
   Builds the Angular app, then the Tauri app. Output is in `src-tauri/target/release/` (or `target/release/bundle/` for installers).

### Project layout

```
grbl-rs/
├── src/                 # Library: machines/grbl, session, profiles
├── src-tauri/           # Tauri app (depends on grbl-rs with "serial" feature)
│   ├── src/lib.rs       # Commands: list_serial_ports, get_mock_status, etc.
│   └── capabilities/
├── angular-ui/          # Angular 19 frontend (default for Tauri)
│   ├── src/app/         # Components, TauriService
│   └── package.json
├── ui/                  # Legacy vanilla frontend (HTML + JS)
├── Cargo.toml           # grbl-rs library
├── package.json         # npm scripts for Tauri CLI
└── README.md
```

## Hardware

Target machine: Genmitsu PROVerXL 4030, BTT Octopus Pro V1.1, GRBL-HAL. USB at 115200 baud.

## License

Private / MeshForge.
