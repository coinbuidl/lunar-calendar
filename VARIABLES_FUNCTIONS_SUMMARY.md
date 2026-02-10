# Variables and Functions Summary

This summary reflects the current split design:
- Lookup helpers: `src/table_lookup.rs`
- Calculation helpers: `src/bazi_calc.rs`
- Public facade/re-exports: `src/lunar-calendar-algo.rs`

## 1) Public Facade: `src/lunar-calendar-algo.rs`

### Module exports
- `pub mod data`
  - Points to `src/lunar-calendar-data-1900-2099.rs`.

### Re-exported public API
- `get_cycle_index`
- `get_stem_branch`
- `get_nayin_by_index`
- `get_nayin_by_year`
- `get_year_data`
- `get_jieqi_context`
- `get_pillars`
- `BaziPillars`
- `JieQiContext`

## 2) Lookup Module: `src/table_lookup.rs`

### Constants
- `STEMS` (`pub(crate)`)
  - 10 Heavenly Stems.
- `BRANCHES` (`pub(crate)`)
  - 12 Earthly Branches.
- `MIN_SUPPORTED_YEAR` / `MAX_SUPPORTED_YEAR` (public)
  - Supported table year bounds (`1900..=2099`).
- `NA_YIN` (private)
  - 30 NaYin entries.

### Functions
- `get_cycle_index(year: i32) -> usize`
  - Convert Gregorian year to 60-cycle index (`1984 -> 0`).

- `get_stem_branch(index: usize) -> Option<(&'static str, &'static str)>`
  - Get stem/branch from cycle index.

- `get_nayin_by_index(index: usize) -> Option<&'static str>`
  - Get NaYin from cycle index.

- `get_nayin_by_year(year: i32) -> &'static str`
  - Get NaYin directly from Gregorian year.

- `get_year_data(year: i32) -> Option<&'static BaziYearData>`
  - Lookup yearly static row from `BAZI_YEAR_DATA`.

## 3) Calculation Module: `src/bazi_calc.rs`

### Structs
- `BaziPillars`
  - `year`, `month`, `day`, `hour`: four pillar strings.
  - `Display`: pretty `年/月/日/时` output.

- `JieQiContext`
  - `prev_name`, `prev_time`: previous solar term.
  - `next_name`, `next_time`: next solar term.
  - `diff_prev_seconds`, `diff_next_seconds`: time deltas.
  - `Display`: human-readable delta summary.

### Functions
- `get_jieqi_context(target_time: NaiveDateTime) -> Option<JieQiContext>`
  - Uses local table years (`year-1`, `year`, `year+1`) to find surrounding JieQi.
  - Uses an internal `OnceLock` parsed-cache to avoid repeated timestamp parsing.

- `get_pillars(dt: NaiveDateTime) -> Option<BaziPillars>`
  - Computes year/month/day/hour pillars using LiChun/Jie boundaries and cycle math.
  - Uses cached parsed table data for faster repeated calls.

### Internal helper
- `format_gz(index: usize) -> String` (private)
  - Builds a `干支` string from cycle index.

## 4) Static Data Module: `src/lunar-calendar-data-1900-2099.rs`

### Structs
- `JieQiData`
  - `name`, `time` (`YYYY-MM-DD HH:MM:SS`, GMT+8)

- `BaziYearData`
  - `year`
  - `lunar_new_year`
  - `lichun_time`
  - `jieqi` (24 terms)

### Constant table
- `BAZI_YEAR_DATA: [BaziYearData; 200]`
  - Full local table for years `1900..=2099`.

## 5) Binaries

### `src/bin/query-lunisolar.rs` (primary table-first CLI)
- `parse_input_datetime(input) -> Result<NaiveDateTime, String>`
  - Parses `YYYY-MM-DD` or `YYYY-MM-DD HH:MM[:SS]`.
- `main()`
  - Prints table lookup data, pillars, and JieQi context.
  - Optional `--verify-with-crate` prints external crate lunar-date verification.

### `src/bin/verify-crate.rs` (verification CLI)
- `parse_input_date(input) -> Result<NaiveDate, String>`
- `main()`
  - Gregorian to lunar date via external crate (for verify/generation workflow).

### `src/main.rs` (demo binary)
- `main()`
  - Example outputs for 60-cycle, NaYin, table rows, pillars, and JieQi context.

## Agent Quick Call Map

- Fast table lookup:
  - `get_year_data(year)`
- Fast local calc:
  - `get_pillars(dt)`
  - `get_jieqi_context(dt)`
- Optional crate verify for exact lunar month/day text:
  - Use `query-lunisolar --verify-with-crate` or `verify-crate`.
