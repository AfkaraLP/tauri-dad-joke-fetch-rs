## Dad Joke Fetcher (Tauri Edition)

Fetch dad jokes in a standalone app... *who wouldn't want that?* This project uses **Tauri** for a lightweight native experience and **Bun** for a lightning-fast development workflow.

---

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

* **[Rust](https://www.rust-lang.org/tools/install)** (The engine powering the backend)
* **[Bun](https://bun.sh/)** (The fast JavaScript runtime & package manager)

### 1. Installation

#### Download Prebuilt Binaries (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/AfkaraLP/tauri-dad-joke-fetch-rs/releases):

- **Linux**: Download the `.deb` package or `.AppImage` file
  - For `.deb`: `sudo dpkg -i dad_jokes_*.deb`
  - For `.AppImage`: Make it executable with `chmod +x dad_jokes_*.AppImage` and run it
- **macOS**: Download the `.dmg` file, open it, and drag the app to your Applications folder
- **Windows**: Download the `.msi` installer or `.exe` and run it

Install the project dependencies (frontend and Tauri CLI):

#### Or Build From Source

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
