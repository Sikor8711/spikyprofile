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

```
spikyprofile.dev/
├── src/
│   ├── app.rs            # Root Leptos app component & router
│   ├── main.rs           # Axum server entrypoint
│   ├── pages/            # Route components (home, about, blog)
│   ├── components/       # Shared UI components
│   └── posts/            # Blog post content & metadata
├── style/
│   └── main.css          # Tailwind entry point
├── public/               # Static assets
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

## Deployment

The site runs on a self-hosted Proxmox server behind Nginx. Deployments are automated via GitHub Actions — push to `main` triggers a build and deploy cycle.

## Author

**Patryk Sikorski**

- [spikyprofile.dev](https://spikyprofile.dev)
- [GitHub](https://github.com/Sikor8711)
- [LinkedIn](https://linkedin.com/in/patryk-sikorski-a4ab7610a)

## License

This project is licensed under the MIT License. Blog content is © Patryk Sikorski — all rights reserved.
