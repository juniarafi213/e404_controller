# E404 Controller (SM8150 / Poco X3 Pro)

Unified hardware, BORE 7.0.1 scheduler, GPU UV/levels, UFS 3.1, bypass charging, uclamp tuning, and auto-game profile controller for Xiaomi Poco X3 Pro (`vayu` / SM8150) running custom kernel `4.19.404R`.

Features an integrated **Material Design 3 WebUI** with Bottom Navigation Bar inspired by **Encore** (clean geek aesthetic, high density, zero CDN, 100% offline).

---

## What's New in v1.2.0

- **Dynamic Profiles (Benchmark vs Performance):**
  - **🚀 Mode Benchmark:** 100% unleashes full hardware potential. Locked 692 MHz GPU (OPP0), 2.96 GHz Prime CPU, rate limit 500µs, uclamp top-app min 50%, BORE slice protect lv2.
  - **🎮 Mode Performance:** Intelligent Dynamic Scaling. Boosts only when frame drops or heavy loads occur; keeps efficient clocks during smooth gameplay (585 MHz GPU cap with full idle down to 150 MHz, balanced CPU ceilings, uclamp top-app min 15%).
  - **⚖️ Mode Balanced:** Full stock dynamic range for 120Hz daily smoothness and optimal efficiency.
  - **🔋 Mode Battery (ROM AOSP Sync):** Activates *only* when the ROM's built-in battery saver is turned on. Frequencies are held at sweet-spot minimums (Silver 1.34 GHz, Gold 1.48 GHz, Prime 1.40 GHz, uclamp max 50%) — eliminating battery drain without causing UI/UX stutter or typing lag.
- **uclamp (Utilization Clamping):**
  - Dynamically configures `/dev/cpuctl/top-app/cpu.uclamp.{min,max,latency_sensitive}` and `background/cpu.uclamp.max` per profile for micro-stutter elimination and power efficiency.
- **Material 3 Bottom Navigation & App Manager:**
  - **Dashboard Tab:** Real-time telemetry, live uclamp monitors, profile cards, and hardware toggles.
  - **Kelola Game Tab:** Lists installed 3rd-party user applications (`pm list packages -3`) with live search filter and instant toggles to add/remove apps to/from auto-detection `gamelist.txt`.
- **System Toast Notifications:**
  - Triggers direct Android notifications / toast alerts on mode transitions and game launches (`cmd notification post`).
- **Event-Driven Daemon (0.0% CPU):**
  - Auto-game and battery saver monitoring via `system_monitor` and kernel `inotify` on `sysmon_out.txt:cw`. Zero CPU overhead during idle and gaming.

---

## Hardware Toggles

- **Bypass Charging:** Powers phone directly from USB without charging battery (`/sys/class/power_supply/battery/input_suspend`).
- **6A Fast Charge Override:** Forces fast charge current under kernel thermal control.
- **Lyb Low-Latency Touch:** Poco X3 Pro low latency touchscreen game mode.

---

## WebUI & Access

- **KernelSU Manager WebUI:** Built-in module WebUI support.
- **Browser WebUI:** `http://127.0.0.1:8404` (hosted by Busybox HTTPD & CGI).
- **WebUI X / MMRL:** Open via module actions.
- **CLI Utility:** `su -c e404-ctrl <command>`.

---

## Installation

Flash `E404_Controller_v1.2.0.zip` in **KernelSU**, **Magisk v20.4+**, or **APatch**.

---

## Credits
- Rem01Gaming for `system_monitor` and `encore` inspiration.
- Google DeepMind Antigravity & E404 Kernel Team.
