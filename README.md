# Fieldstack

A desktop field service management application built with **Tauri v2**, **SvelteKit**, and **TypeScript**.

## Features

- **Dashboard** — At-a-glance stats, overdue job banner, activity feed, and quick filters
- **Job Board** — Create, track, and manage field service jobs with a 5-step wizard, draft auto-save, proof file attachments, dispute/resolve workflow, and overdue detection
- **Organization** — Manage companies and contacts (people) linked to jobs
- **Reports** — Job and org reporting views with completion charts
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
│   │   ├── +page.svelte        # Root navigation shell
│   │   ├── dashboard/
│   │   ├── jobs/
│   │   ├── org/
│   │   ├── reports/
│   │   └── settings/
│   └── lib/
│       ├── components/
│       │   ├── organisms/      # Sidebar, JobWizard, JobTable, JobDetails
│       │   └── primitives/     # Button, Badge, StatCard, Skeleton, EmptyState
│       ├── stores/             # Svelte stores (jobs, ui, dashboard)
│       ├── types/              # TypeScript interfaces
│       └── utils/              # Typed Tauri invoke helpers
├── src-tauri/                  # Tauri / Rust backend
│   ├── src/
│   │   ├── commands/           # Tauri command handlers (jobs, org, proofs, drafts, reports)
│   │   ├── db/                 # SQLite schema and init
│   │   ├── overdue.rs          # Background overdue poller
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## Database

SQLite database is created automatically on first launch at the app data directory. Tables: `companies`, `people`, `person_companies`, `jobs`, `proofs`, `audit_log`, `job_drafts`, `job_counter`, `app_config`.

Seed data is inserted on first run: 2 companies and 3 people.

## License

MIT — see [LICENSE](LICENSE) for details.
