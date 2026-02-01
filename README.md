# embedded-stm32

A collection of STM32 embedded Rust examples covering GPIO, ADC, DMA, I2C, SPI, UART, PWM, and more. Each subdirectory is an independent example that can be built and flashed; this repository is intended for learning, demos, and quick prototyping of STM32 peripherals.

---

## Table of Contents 📁
- Overview
- Quick Start
- Build & Flash (Common Commands)
- Selected Subprojects

> **Note**: Each subproject usually includes its own `README.md`, `Cargo.toml`, and `memory.x`. Please check the subproject README for example-specific instructions.

---

## Quick Start 🚀
1. Install Rust (stable or the toolchain required by a specific example):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Add a target triple (example):
   ```bash
   rustup target add thumbv7m-none-eabi
   # or thumbv6m-none-eabi depending on the MCU
   ```
3. Useful tools (optional):
   - `probe-run`: `cargo install probe-run`
   - `cargo-embed`: `cargo install cargo-embed`
   - `openocd` / `st-flash`: install via your package manager

---

## Build & Flash (Common Commands) 🔧
- Build:
  ```bash
  cd <example-dir>
  cargo build --release
  ```
- Run with `probe-run`:
  ```bash
  cargo run --release
  ```
- Flash and debug with `cargo-embed`:
  ```bash
  cargo embed --release
  ```

> **Note**: Commands may vary depending on the example's `Cargo.toml` runner configuration and the target MCU; consult the subproject README.

---

## Selected Subprojects 📂
- `01_led_blink` — LED blink example (quick start)
- `adc_mul`, `adc_sig` — ADC examples
- `pwm_servo`, `pwm-led` — PWM examples
- `iic_oled`, `iic_mpu_*` — I2C peripheral examples
- `spi_w25_*` — SPI NOR flash examples
- `usart*` — UART related examples
(See the repository folders for more examples)
---
