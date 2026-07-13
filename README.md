# Slate

Slate is a selfhosted suite of project management tools. Oh also its FOSS!

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://choosealicense.com/licenses/mit/)
[![GitHub issues](https://img.shields.io/github/issues/kashsuks/slate)](https://github.com/kashsuks/slate/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/kashsuks/slate)](https://github.com/kashsuks/slate/commits/master)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri-24C8DB?logo=tauri&logoColor=white)](https://tauri.app)
[![Built with Svelte](https://img.shields.io/badge/built%20with-Svelte-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev)

## Status

Currently under active development, not yet stable. Once ready for beta testing, desktop builds will be published on GitHub Releases and the server image on Docker.

## Standalone (desktop app)

Download it [here](https://github.com/kashsuks/slate/releases)

The standalone version is a Tauri desktop app with its own embedded SQLite database, no server required.

Requirements: [Bun](https://bun.sh), [Rust](https://www.rust-lang.org/tools/install), and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS.

```bash
bun install
bun run tauri dev
```

To build a native binary/installer:

```bash
bun run tauri build
```

Data is stored in your OS's app data directory (e.g. `~/.local/share/com.ksukshavasi.tauri-app` on Linux, `~/Library/Application Support/com.ksukshavasi.tauri-app` on macOS).

## Server (selfhosted)

The server version runs the same Rust binary headlessly behind a REST API, with SvelteKit built as a Node server frontend. This is what you'd run on a home server or VPS.

1. Build the image

```bash
docker build -t slate .
```

2. Run the image

```bash
docker run -d \
  -p 3000:3000 \
  -v /your/data/path:/data \
  --name slate \
  slate
```

Environment variables:

| Variable         | Default        | Description                    |
|------------------|----------------|--------------------------------|
| `SLATE_PORT`     | `3000`         | Port the server listens on     |
| `SLATE_DB_PATH`  | `/data/slate.db` | Path to the SQLite database  |

## Keyboard shortcuts

Press `?` in the app anytime to bring up the shortcuts overlay. Current shortcuts:

| Keys      | Action                        |
|-----------|-------------------------------|
| `N` `1-9` | Add a card to column N        |
| `B`       | Create a new board            |
| `C`       | Create a new column           |
| `Esc`     | Close modal / cancel input    |
| `?`       | Toggle the shortcuts overlay  |

## Things to look out for

- This project is pre-beta: expect schema/data-format changes between commits until a release is tagged.
- The standalone and server builds share the same Rust backend and SQLite schema, but keep separate databases, moving between them isn't automatic.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
