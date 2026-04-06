use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn HwmonitorPartOnePost() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Hardware Monitor Part 1 — Concept" />
        <Title text="Spiky Profile - Hardware Monitor Part 1 — Concept" />
        <div class="text-neutral-300 space-y-5">
            <h1 class="text-2xl md:text-3xl text-center">"Hardware Monitor Part 1 — Concept"</h1>
            <div class="space-y-5">
                <p>
                    "I'm building a hardware monitoring system in Rust. Not a dashboard that reads \"/proc/stat\" on one machine — a distributed system where multiple machines report their hardware data to a central receiver, which then serves it to a web frontend in real time."
                </p>
                <p>
                    "This is Part 1. No code yet. Just the architecture and the reasoning behind it."
                </p>
                <h2 class="text-xl font-bold">"The Problem"</h2>
                <p>
                    "I run a Proxmox homelab. Multiple VMs, multiple services, and I want to see what's happening across all of them from one place. Existing tools either do too much (Grafana + Prometheus stack for a homelab?) or too little (htop on each machine via SSH)."
                </p>
                <p>
                    "I want something lightweight, purpose-built, and — most importantly — something I understand from the ground up because I wrote it."
                </p>
                <h2 class="text-xl font-bold">"The Architecture"</h2>
                <p>"The system has three roles that all live inside one binary:"</p>
                <ul class="list-disc list-inside">
                    <li>
                        <b>"Sender "</b>
                        "- runs on each machine you want to monitor. Collects hardware data (CPU, RAM, temps, disk) and exposes it when asked."
                    </li>
                    <li>
                        <b>"Receiver "</b>
                        "— runs on one machine. Knows about all the senders. When activated, it polls each sender, aggregates the data, and pushes it to the requesting client."
                    </li>
                    <li>
                        <b>"WebSocket endpoint "</b>
                        "— the persistent connection between the receiver and the frontend (a Leptos component on spikyprofile.dev)."
                    </li>
                </ul>
                <p>
                    "
                    Every instance also gets a "<b>"TUI powered by Ratatui"</b>
                    " — a terminal interface that lets you see what's happening on that node in real time. Whether it's a sender showing its local hardware stats or a receiver showing aggregated data from all senders, you get a live view right in your terminal. No browser needed to check on things. SSH in, see the state, get out.
                    "
                </p>
                <p>
                    "One binary. You configure it as a sender, a receiver, or both. That's the first design decision and it matters — no juggling different packages or versions across machines."
                </p>
                <p>"Here's the full architecture:"</p>
                <img
                    class="max-h-100 mx-auto"
                    src="/assets/images/hwmonitor_concept.png"
                    alt="Concept of design"
                />
                <p>
                    "The orange-bordered node is the primary receiver — the hub connected to the webpage. The red-highlighted Sender and TCP blocks show which layers handle outbound data on each node. Every node runs the same binary with the same internal structure: TUI on top, Sender/Receiver logic, TCP/WebSocket transport, and Axum with auth at the base."
                </p>
                <h2 class="text-xl font-bold">"The Flow"</h2>
                <p>
                    "The website drives the lifecycle. Nothing runs until someone is actually looking at the page. No wasted cycles, no background daemons burning resources when nobody cares."
                </p>
                <ul class="list-decimal list-inside">
                    <li>"The Leptos component mounts → opens a WebSocket to the receiver."</li>
                    <li>
                        "The receiver begins its polling loop — requesting data from every registered sender."
                    </li>
                    <li>"Each sender responds with its current hardware snapshot."</li>
                    <li>
                        "The receiver aggregates everything and pushes it down the WebSocket as JSON."
                    </li>
                    <li>"This repeats on an interval until..."</li>
                    <li>
                        "The component unmounts (user navigates away or closes the tab) → WebSocket connection drops."
                    </li>
                    <li>"The receiver detects the closed connection and stops polling. Done."</li>
                </ul>
                <p>"No explicit stop endpoint needed. The connection is the lifecycle."</p>
                <p>
                    "Meanwhile, on any node, you can open the Ratatui TUI to see live data locally — independent of the web frontend."
                </p>
                <h2 class="text-xl font-bold">"Why This Design?"</h2>
                <p>
                    <b>"WebSockets, not REST polling."</b>
                </p>
                <p>
                    "The frontend needs continuous data on a short interval. Opening a new HTTP request every few seconds means repeated handshakes, headers, connection setup — all wasted work when both sides already know they're talking to each other. A WebSocket holds that connection open. The receiver pushes data the moment it's ready. And when the user leaves, the dropped connection is the stop signal — clean, automatic, no extra logic."
                </p>
                <p>
                    <b>"One binary, multiple roles."</b>
                </p>
                <p>
                    "This is a Rust decision. A CLI flag or config file switches behaviour. Compile once, deploy everywhere. The sender on a lightweight VM doesn't need to carry receiver logic (or it can — your call). Deployment is dead simple: scp the binary, write a config, run it."
                </p>
                <p>
                    <b>"Senders don't push — they respond."</b>
                </p>
                <p>
                    "Senders are passive. They don't need to know where the receiver is or manage any outbound connections. The receiver asks, the sender answers. Adding a new machine to the monitor is just: deploy the binary as a sender, add its address to the receiver's config. No coordination, no discovery protocol, no added complexity."
                </p>
                <p>
                    <b>"Ratatui for the terminal interface."</b>
                </p>
                <p>
                    "Every node gets a TUI out of the box. This isn't just a nice-to-have — it's how you'll interact with the system 90% of the time. SSH into a machine, fire up the TUI, see live stats. No need to open a browser or hit an API. Ratatui gives us a rich, responsive terminal UI with graphs, tables, and real-time updates — all without leaving the terminal where you're already working."
                </p>
                <h2 class="text-xl font-bold">"The Stack"</h2>
                <ul class="list-disc list-inside">
                    <li>
                        <b>"Axum "</b>
                        " — HTTP server and WebSocket handling"
                    </li>
                    <li>
                        <b>"Sysinfo "</b>
                        " — cross-platform hardware data collection"
                    </li>
                    <li>
                        <b>"Ratatui "</b>
                        " — terminal user interface"
                    </li>
                    <li>
                        <b>" Tokio"</b>
                        " — async runtime tying it all together"
                    </li>
                    <li>
                        <b>"Leptos "</b>
                        " — the web frontend that consumes the data (on spikyprofile.dev)"
                    </li>
                </ul>
                <h2 class="text-xl font-bold">"What's Coming in Part 2"</h2>
                <p>
                    "The actual implementation. I'll start with the sender — reading hardware data using the sysinfo crate, exposing it over a simple HTTP endpoint with Axum, and wiring up a basic Ratatui display so you can see your own machine's stats in the terminal. That's the smallest useful piece, and it works standalone before we even think about the receiver."
                </p>

                <p>
                    "If you want to follow along, you'll need Rust installed and a machine with a pulse (CPU temps count)."
                </p>
                <hr />
                <p class="italic text-neutral-500 font-light">
                    "This is the first post in the Hardware Monitor series. I'm building this in public as part of Spiky Rust — learning Rust by building real things, explained from the top down."
                </p>
            </div>
            <div class="flex justify-between">
                <p>"Author: Patryk Sikorski"</p>
                <p>"Posted: 06/04/2026"</p>
            </div>
        </div>
    }
}
