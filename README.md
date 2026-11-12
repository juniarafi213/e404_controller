# E404 Controller (SM8150 / Poco X3 Pro)

Unified hardware, BORE 7.0.1 scheduler, GPU UV/levels, UFS 3.1, bypass charging, and auto-game profile controller for Xiaomi Poco X3 Pro (`vayu` / SM8150) running custom kernel `4.19.404R`.

Features an integrated **Material Design 3 WebUI** inspired by **Encore** (clean geek aesthetic, high density, zero CDN, 100% offline).

---

## Features

- **Auto Game Mode Daemon:** Detects foreground game applications via `system_monitor` with kernel `inotify` (0.0% CPU overhead while playing/idle).
- **Performance Lite Toggle:** When active, automatically locks game profile to `Game Lite` (585 MHz GPU cap) preventing severe DDR RAM 2133 MHz battery drain in heavy games like Zenless Zone Zero (ZZZ).
- **4 Tuned Hardware Profiles:**
  - `Performance`: Uncapped 692 MHz GPU, unconstrained CPU, BORE slice protect lv2, low-latency schedutil 1000µs, Lyb touch game mode.
  - `Game Lite`: 585 MHz GPU cap (saves battery & stops DDR bus locking), Prime 2.45 GHz, Gold 2.15 GHz, BORE slice protect lv1, Lyb touch game mode.
  - `Balanced`: Stock dynamic clocks, 585 MHz cap with full idle to 150 MHz, standard thermals, smooth 120Hz daily driver.
  - `Battery`: Throttled clocks (1.75 GHz Prime, 1.67 GHz Gold, 1.51 GHz Silver, 427 MHz GPU), delayed schedutil ramp 8000µs.
- **Hardware Toggles:**
  - **Bypass Charging:** Powers phone directly without charging battery (`/sys/class/power_supply/battery/input_suspend`).
  - **6A Fast Charge Override:** Forces fast charge current under kernel thermal control.
  - **Lyb Low-Latency Touch:** Poco X3 Pro low latency touchscreen mode.
- **Material Design 3 WebUI:**
  - Works natively inside KernelSU Manager WebUI.
  - Works in browser via `http://127.0.0.1:8404` (hosted by Busybox HTTPD & CGI).
  - Works via WebUI X / MMRL.
- **CLI Utility:** `su -c e404-ctrl <command>`.

---

## Installation

Flash `E404_Controller_v1.1.0.zip` in **KernelSU**, **Magisk v20.4+**, or **APatch**.

---

## Credits
- Rem01Gaming for `system_monitor` and `encore` inspiration.
- Google DeepMind Antigravity & E404 Kernel Team.
