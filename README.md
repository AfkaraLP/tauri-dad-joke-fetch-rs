## Dad Joke Fetcher (Tauri Edition)

Fetch dad jokes in a standalone app... *who wouldn't want that?* This project uses **Tauri** for a lightweight native experience and **Bun** for a lightning-fast development workflow.

---

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

* **[Rust](https://www.rust-lang.org/tools/install)** (The engine powering the backend)
* **[Bun](https://bun.sh/)** (The fast JavaScript runtime & package manager)

### 1. Installation

Install the project dependencies (frontend and Tauri CLI):

```bash
bun install
```

### 2. Development

Launch the app in development mode. This will start the frontend dev server and compile the Rust backend with hot-reloading enabled.

```bash
bun tauri dev
```

### 3. Build

Create a production-ready, optimized bundle for your operating system (e.g., `.msi`, `.dmg`, or `.deb`):

```bash
bun tauri build
```

The final installer will be located in:
`src-tauri/target/release/bundle/`

---

## Automated Builds

This project includes a GitHub Actions workflow that automatically builds executables for multiple platforms:

- **Linux** (x86_64) - `.deb` and `.AppImage`
- **macOS** (Intel x86_64) - `.dmg` and `.app`
- **macOS** (Apple Silicon ARM64) - `.dmg` and `.app`
- **Windows** (x86_64) - `.msi` and `.exe`

The workflow runs on:
- Push to `main` or `master` branches
- Pull requests to `main` or `master` branches
- Manual trigger via workflow dispatch

Build artifacts are automatically uploaded and can be downloaded from the GitHub Actions run.
