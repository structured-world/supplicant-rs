# supplicant-rs

Pure Rust WPA supplicant. Memory-safe replacement for wpa_supplicant.

**WPA2/WPA3, EAP-TLS, 802.1X — no C, no unsafe, no CVEs.**

## Features

- **WPA2** — 4-way handshake, PMKSA caching
- **WPA3-SAE** — Simultaneous Authentication of Equals (dragonfly)
- **EAP** — EAP-TLS (via rustls), EAP-TTLS, EAP-PEAP
- **802.1X** — EAPOL state machine
- **PMF** — Protected Management Frames
- **nl80211** — Linux kernel Wi-Fi interface via netlink

## Why?

wpa_supplicant is ~500K lines of C running as root on every Linux device with Wi-Fi. It parses complex (and often malicious) protocol frames. Memory corruption vulnerabilities like KRACK (CVE-2017-13077) affect billions of devices.

`supplicant-rs` is a ground-up Rust rewrite. Memory-safe by construction. No `unsafe` in protocol handling.

## Status

**Early development.** Not functional yet. Seeking Prossimo/ISRG funding for full implementation.

## License

Apache-2.0
