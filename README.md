# spikyprofile.dev

A personal blog and portfolio built with [Leptos](https://leptos.dev/) and [Axum](https://github.com/tokio-rs/axum) — fully server-side rendered, self-hosted on bare-metal infrastructure.

Home of the **Spiky Rust** tutorial series: top-down, context-first Rust teaching for developers who already code.

## About

This site is the public face of my transition from knitting machine operator to software engineer. The name comes from my formal cognitive assessment, which revealed a striking "spiky profile" — outstanding silent reading comprehension alongside well-below-average decoding and working memory. That contrast shapes how I learn, how I teach, and how I build.

The blog covers:

- **Spiky Rust** — a tutorial series that starts with the end state and works backward, prioritising intuition before formalism. Written for developers who already know how to code and want to understand Rust without wading through toy examples.
- **Self-hosting & infrastructure** — Proxmox, Nginx, CI/CD, and the reality of running your own stack.
- **Career transition** — honest writing about the junior developer job search, 40+ rejections, and what the process actually looks like.

## Tech Stack

| Layer         | Technology              |
| ------------- | ----------------------- |
| Frontend      | Leptos (SSR)            |
| Backend       | Axum                    |
| Language      | Rust                    |
| Styling       | Tailwind CSS            |
| Hosting       | Bare-metal Proxmox      |
| Reverse Proxy | Nginx                   |
| CI/CD         | GitHub Actions          |
| Analytics     | Plausible (self-hosted) |

## Project Structure

```text
spikyprofile.dev/
├── src/
│   ├── app.rs            # Root Leptos app component & router
│   ├── main.rs           # Axum server entrypoint
│   ├── pages/            # Route components (home, about, blog)
│   ├── components/       # Shared UI components
│   └── posts/            # Blog post content & metadata
├── style/
│   └── main.scss         # custom css
│   └── main.css          # Tailwind entry point
├── assets/               # Static assets
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
- [Tailwind CSS CLI](https://tailwindcss.com/blog/standalone-cli)

### Development

```bash
# Clone the repo
git clone https://github.com/Sikor8711/spikyprofile.dev.git
cd spikyprofile.dev

# Install tailwindcss
npm install

# Install cargo-leptos if you haven't already
cargo install cargo-leptos

# Run the dev server with hot reload
cargo leptos watch
```

The site will be available at `http://localhost:3000`.

### Production Build

```bash
cargo leptos build --release
```

The compiled binary will be in `target/server/release/`.

## 🚀 Architecture & Deployment

The site is hosted on a private bare-metal server using a secure, automated CI/CD pipeline designed with strict least-privilege principles.

### Infrastructure

- **Host Environment:** Bare-metal Proxmox server operating on a 1Gbps symmetric fiber connection.
- **Network Isolation:** The Proxmox host holds the single public IP. The application runs inside a sandboxed Virtual Machine (VM) on a completely private, local subnet (`192.168.1.x`).
- **Ingress Routing:** Traffic is routed using an **Nginx stream block** (Layer 4 proxying) on the host node, transparently forwarding raw TCP/UDP packets directly to the internal VM without exposing the host itself.

### CI/CD Pipeline (GitHub Actions)

Deployments are fully automated. Pushing code to the `main` branch triggers a complete build and deploy cycle utilizing a Bastion/Jump Host architecture:

- **The Build Phase:** GitHub Actions provisions an Ubuntu runner, installs the Rust nightly toolchain, caches dependencies, and compiles the Leptos WASM payload and Axum release binary.
- **The Proxy Jump:** Because the webserver VM is isolated on a private subnet, GitHub Actions authenticates via SSH into the public-facing Proxmox Host (the Bastion). The connection is then seamlessly tunneled through the Bastion directly into the internal webserver VM.
- **Least Privilege Delivery:** The automated worker does not authenticate as `root`. It logs into the webserver as a restricted, non-privileged user and SCPs the compiled artifacts into a temporary staging directory (`/tmp`).

### The Deployment Hand-off

The final deployment phase is handled by a local bash script (`deploy_sp.sh`) on the VM. This script acts as a secure bridge between the staging area and the live environment.

To maintain security without requiring manual password entry, the restricted deployment user is granted explicitly scoped permissions via the `/etc/sudoers` file to run _only_ this specific bash script as root (`NOPASSWD`).

When triggered by GitHub Actions, the script executes the following atomic update:

1. **Binary Relocation:** Moves the newly compiled Rust binary from `/tmp` to the protected web root (`/var/www/html/...`) and applies executable permissions.
2. **Asset Refresh:** Wipes the live `public` directory and copies over the newly compiled WASM, CSS, and static assets.
3. **Service Restoration:** Restarts the underlying `systemd` daemon (`sp.service`) to spin up the new Axum process and immediately verifies the active service status.

## Author

**Patryk Sikorski**

- [spikyprofile.dev](https://spikyprofile.dev)
- [GitHub](https://github.com/Sikor8711)
- [LinkedIn](https://linkedin.com/in/patryk-sikorski-a4ab7610a)

## License

This project is licensed under the MIT License. Blog content is © Patryk Sikorski — all rights reserved.
