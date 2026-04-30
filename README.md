# Fieldstack

A desktop field service management application built with **Tauri v2**, **SvelteKit**, and **TypeScript**.

## Features

- **Job Board** — Create, track, and manage field service jobs with a 5-step wizard, draft auto-save, proof file attachments, dispute/resolve workflow, and overdue detection
- **Organization** — Manage companies and contacts (people) linked to jobs
- **Dashboard** — At-a-glance stats, overdue job banner, and quick filters
- **Reports** — Job and org reporting views
- **Settings** — App configuration

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | Tauri v2 (Rust) |
| Frontend | SvelteKit 2 + Svelte 5 |
| Styling | Tailwind CSS v4 |
| Icons | Phosphor Svelte |
| Database | SQLite via Tauri commands |
| Language | TypeScript |

## Prerequisites

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://rustup.rs/) (stable toolchain)
- [Tauri CLI prerequisites](https://tauri.app/start/prerequisites/) for your OS

## Getting Started

```bash
# Install dependencies
npm install

# Run in development mode (opens the desktop app)
npm run tauri dev

# Type-check the frontend
npm run check
```

## Building

```bash
# Build the desktop app (produces a native installer in src-tauri/target/release/bundle/)
npm run tauri build
```

## Project Structure

```
fieldstack/
├── src/                        # SvelteKit frontend
│   ├── routes/
│   │   ├── +page.svelte        # Root / navigation shell
│   │   ├── dashboard/
│   │   ├── jobs/
│   │   ├── org/
│   │   ├── reports/
│   │   └── settings/
│   └── lib/
│       ├── components/         # Shared UI components
│       ├── stores/             # Svelte stores
│       ├── types/              # TypeScript types
│       └── utils/              # Tauri invoke helpers
├── src-tauri/                  # Tauri / Rust backend
│   ├── src/
│   │   ├── commands/           # Tauri command handlers
│   │   ├── db/                 # SQLite database layer
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## License

MIT — see [LICENSE](LICENSE) for details.
