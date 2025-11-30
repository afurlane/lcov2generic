# lcov2generic

[![Crates.io](https://img.shields.io/crates/v/lcov2generic.svg)](https://crates.io/crates/lcov2generic)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/<tuo-utente>/lcov2generic/actions/workflows/ci.yml/badge.svg)](https://github.com/<tuo-utente>/lcov2generic/actions)

Convertitore da **LCOV** a **SonarQube Generic Coverage XML** per la Community Edition.  
Permette di integrare facilmente la copertura dei test Rust in SonarQube/SonarCloud anche quando non è disponibile il supporto nativo.

---

## ✨ Funzionalità
- Legge un file `lcov.info` generato da `cargo-llvm-cov` o `grcov`.
- Converte line coverage in formato **Generic Coverage XML** (`<coverage>`).
- Output pronto da passare a SonarQube tramite `sonar.coverageReportPaths`.
- Semplice da integrare in CI/CD (Makefile, GitHub Actions, GitLab CI).

---

## 🚀 Installazione
Da [crates.io](https://crates.io/crates/lcov2generic):
```bash
cargo install lcov2generic
