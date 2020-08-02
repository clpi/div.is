# div.is

### to run front:
`cd client && npm run dev`

SPA on localhost:5000, SSR on localhost:5005. uses Svelte + Routify

### to run back:
`cd back && cargo run`
or
`cd back && cargo-watch -x run`

API on localhost:3001. Uses Warp + sqlx

## db:
wip (using sqlite right now)

### deploying:
Currently using Podman and Ansible to deploy to DigitalOcean

### recent changes:
- $08/01/20$ ditched yew app in this repo for now. using yew on personal website and feeling it out alongside other wasm libraries in rust
- $08/01/20$ switched to using svelte + TS (new) alongside Tauri and Neon Node.js Rust bindings to create as fast and native of a web-rendered desktop app as I can... hopefully!
    - $08/02/20$ maybe look into flutter-rs as an early adopter?

### to do
