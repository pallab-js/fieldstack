# Fieldstack macOS Desktop Client: Technical Blueprint

This blueprint outlines the development architecture for the **Fieldstack** macOS desktop client. The project aims to provide a high-performance, native-feeling administrative interface for Gautam to manage his fertilizer and construction businesses.

---

## 1. Project Overview
The Fieldstack macOS client is a **native desktop application** designed specifically for Apple Silicon (M1/M2/M3). It serves as the "God View" for the owner, enabling organization building, workforce tracking, and financial audit capabilities.

### Primary Objectives
- **Visibility:** Real-time tracking of field sales and warehouse operations.
- **Accountability:** Anti-fraud procurement verification for construction sites.
- **Performance:** Optimized for MacBook Air M1 (8GB RAM) with minimal memory footprint.

---

## 2. Technology Stack
The chosen stack leverages the efficiency of Rust and the developer velocity of SvelteKit.

| Layer | Technology | Reason for Selection |
| :--- | :--- | :--- |
| **Runtime** | **Tauri 2.0** | Uses system WebKit (Safari engine) instead of Chromium, drastically reducing RAM usage on 8GB M1 Macs. |
| **Backend** | **Rust** | Type-safe, memory-efficient, and provides native performance for data processing and system integration. |
| **Frontend** | **SvelteKit** | Minimal runtime overhead, excellent developer experience, and fast HMR (Hot Module Replacement). |
| **Styling** | **Tailwind CSS** | Utility-first styling that aligns perfectly with the provided Figma design system. |
| **Build Tool** | **Vite** | Extremely fast build times, critical for the M1's unified memory architecture. |

---

## 3. Architecture & Design

### Core Modules (Owner View)
Based on the design analysis, the desktop client will implement the following high-level modules:

1.  **Dashboard (The "N" View):** A birds-eye overview of both companies, highlighting active jobs, overdue tasks, and anomalies.
2.  **Org Builder:** Hierarchical management for levels (Executive, Manager, Worker) and teams.
3.  **Macro Gantt:** A timeline view of all organizational activities with drill-down capabilities.
4.  **Audit Center:** Reviewing proof submissions (Photos, Voice, Documents) and resolving disputes.

### Data Flow Pattern
- **Frontend (SvelteKit):** Handles UI state, routing, and user interactions.
- **Tauri Commands:** Acts as the bridge, invoking Rust functions for sensitive operations (file I/O, secure storage, heavy data processing).
- **Rust Backend:** Manages local caching, SQLite integration for offline support, and API communication with the server.

---

## 4. Development Workflow (Vibecoding on M1)

To optimize for an 8GB M1 MacBook Air, follow this "Vibecoding" setup in your terminal:

### Environment Setup
```bash
# Install Rust and Tauri CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli --version "^2.0.0"

# Initialize Project
pnpm create tauri-app fieldstack-desktop
# Select: SvelteKit -> TypeScript -> Tailwind CSS
```

### Performance Optimization for 8GB RAM
- **Use `pnpm`:** Faster and more disk-efficient than npm/yarn.
- **Incremental Compilation:** Rust's `cargo` handles this well, but ensure you use `lld` linker for faster macOS builds.
- **Tauri Config:** Enable `prerender` for static pages to improve initial load time.

---

## 5. UI/UX Implementation (Figma to Code)

The UI should strictly follow the **NeoAgriCare Design System** tokens:

### Color Palette (Dark Mode First)
- **Base BG:** `#08090a`
- **Panel BG:** `#0f1011`
- **Accent:** `#5e6ad2` (Fieldstack Purple)
- **Status Colors:** Success (`#27a644`), Overdue (`#ef4444`), Progress (`#f59e0b`).

### Key Components
- **Navigation Sidebar:** Persistent on the left for quick module switching.
- **Contextual Drill-down:** Using Svelte's transition patterns to "zoom" into data levels (Company -> Team -> Person -> Job).
- **Native macOS Features:** Implement Menu Bar extras (system tray) for quick status updates and native notifications.

---

## 6. Implementation Roadmap

### Phase 1: Foundation (Week 1)
- Set up Tauri + SvelteKit boilerplate.
- Implement Tailwind configuration with Figma tokens.
- Build the core navigation shell and layout.

### Phase 2: Org & Data (Week 2)
- Implement the Org Builder UI.
- Create Rust commands for fetching/syncing organizational data.
- Setup local SQLite for offline-first capabilities.

### Phase 3: Dashboard & Gantt (Week 3)
- Build the Dashboard with real-time anomaly alerts.
- Implement the Macro Gantt using a high-performance canvas or optimized SVG.
- Add "New Job" contextual creation flow.

### Phase 4: Audit & Polish (Week 4)
- Build the Proof Viewer (Photo/Video/Voice playback).
- Implement macOS-specific polish (Native title bar, shortcuts).
- Final performance profiling on M1.

---

## 7. Security & Compliance
- **Authentication:** Secure storage of session tokens using the macOS Keychain via Tauri's `stronghold` or native crates.
- **Data Privacy:** Local encryption for cached field worker data (DPDP Act compliance).
- **Audit Trail:** Immutable logging of all owner actions in the desktop client.
