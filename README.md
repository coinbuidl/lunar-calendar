# lunar-calendar

- Rust tools for Chinese lunar calendar, 60 JiaZi cycle, NaYin, BaZi pillars, and JieQi context.
- Designed for AI agent.

## Priority Design

- Primary runtime path is **local table lookup + local algorithm** (`src/lunar-calendar-data-1900-2099.rs` + `src/lunar-calendar-algo.rs`).
- This path is fast and fully offline for supported years (`1900..=2099`).
- External crate path is kept as **verification/generation helper**, not the primary business path.
- Internal structure is split for agent-friendly usage:
- `src/table_lookup.rs` for direct lookup helpers.
- `src/bazi_calc.rs` for JieQi/BaZi calculations.
- `src/lunar-calendar-algo.rs` as a small re-export facade.

## What This Program Can Do

- Print the full 60 JiaZi (`干支`) cycle with NaYin (`纳音五行`).
- Compute NaYin by Gregorian year.
- Query built-in year data (lunar new year date and LiChun timestamp for supported years).
- Compute BaZi pillars (`年柱 月柱 日柱 时柱`) for a given datetime.
- Find surrounding JieQi (previous and next solar term) and time distance from a target datetime.
- Verify Gregorian date -> lunar date (`农历`) with external crate.

## Binaries

- `lunar-calendar` (default run): Main demo output for cycle table, sample queries, BaZi, and JieQi context.
- `query-lunisolar`: **Primary table-first query tool** (offline): year table fields, BaZi pillars, JieQi context.
- `verify-crate`: External crate verification for Gregorian date -> lunar date conversion.

## Usage

Build and run default binary:

```bash
cargo run
```

Run `query-lunisolar` (primary, table-first):

```bash
cargo run --bin query-lunisolar -- 1985-04-21
cargo run --bin query-lunisolar -- "1985-04-21 08:00"
cargo run --bin query-lunisolar -- "1985-04-21 08:00" --verify-with-crate
```

Agent fast path (build once, no rebuild per call):

```bash
cargo build --bin query-lunisolar
./target/debug/query-lunisolar "1985-04-21 08:00"
```

Accepted input format for `query-lunisolar`:

- `YYYY-MM-DD`
- `YYYY-MM-DD HH:MM`
- `YYYY-MM-DD HH:MM:SS`

Run crate verification helper:

```bash
cargo run --bin verify-crate
cargo run --bin verify-crate -- "1985-04-21 08:00"
```

## Example Result

For input `1985-04-21 08:00`, `query-lunisolar` returns local-table/local-algo output such as:

- Lunar new year (table year field): `1985-02-20`
- BaZi: `乙丑年 庚辰月 庚寅日 庚辰时`
- JieQi context around input datetime

If you also want exact lunar month/day text, run crate verification mode:

- `cargo run --bin query-lunisolar -- "1985-04-21 08:00" --verify-with-crate`
- or `cargo run --bin verify-crate -- "1985-04-21 08:00"`

## Project Files

- Main binary: `src/main.rs`
- Core logic: `src/lunar-calendar-algo.rs`
- Lookup module: `src/table_lookup.rs`
- Calc module: `src/bazi_calc.rs`
- Static data table: `src/lunar-calendar-data-1900-2099.rs`
- Date conversion demo: `src/bin/query-lunisolar.rs`
- Crate verification: `src/bin/verify-crate.rs`
