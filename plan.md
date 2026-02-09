# Plan: Enhancing Lunar Calendar Data (1900-2099)

## 1. Goal
Expand `lunar-calendar-data-1900-2099.rs` to include critical markers for high-precision Bazi calculation: the precise start date of each Lunar Year and the entry dates/times for all 24 Solar Terms (Jieqi). A table will be made, so Master can search the four pillar of the date and time with the distance to the previouse and the next jieqi.

## 2. Data Structure Enhancements

### A. Year Start Markers
Instead of just counting days, each year entry should explicitly store:
- `lunar_new_year_date`: The Gregorian date for the 1st day of the 1st Lunar month.
- `lichun_date`: The Gregorian date/time for "Start of Spring" (true beginning of the Bazi year).

### B. Jieqi (Solar Terms) Entry Points
To accurately determine the "Month Pillar," we need the crossover points:
- Store an array or map of all 24 Jieqi per year. 
- Focus on the 12 "Jie" (Sectional Terms) that trigger the change of the Month Stem-Branch.
- Given a date, it is easy to search how far it is from the prevous jieqi and the next jieqi.
- First solar term,"lichun_date",triggers the cahnge of the year stem-branch.

## 3. Implementation Steps

### Phase 1: Data Source (Prefer Existing, Verified Data)
- **Primary choice (recommended):** use 6tail’s "lunar" project as the reference implementation (widely used across Java/Python/Swift/PHP). It uses timezone **GMT+8** by default and provides solar term boundaries and BaZi-related outputs.
- We will **not** hand-roll astronomy from scratch unless needed.

### Phase 2: Table Generator (Offline → Rust Static Data)
- Write a generator script that:
  - iterates years **1900-2099**
  - for each year, exports:
    - `lunar_new_year_date` (农历正月初一的公历日期)
    - `lichun_date` (立春精确时间，作为换年点)
    - all **24 jieqi timestamps** for that year
  - formats them into a Rust static table (new struct) to become our authoritative `lunar-calendar-data-1900-2099.rs`.
- This keeps runtime **pure Rust + zero external dependencies**, while the generation step leverages a proven data source.

### Phase 3: Query Functions (Distance Engine)
- Implement:
  - given a datetime → previous jieqi / next jieqi + precise duration
  - given a datetime → year/month pillar boundary checks (based on lichun + 12 Jie)

### Phase 2: Data Regeneration
- Re-run the algorithm for the 1900-2099 range.
- Update `lunar-calendar-data-1900-2099.rs` with the new expanded struct format.

### Phase 3: Binary Optimization
- Update `main.rs` to allow querying:
    - "When does the wood dragon year actually start?"
    - "What is the crossover date for the Yin Wood Rabbit month in 1987?"
- Perform a `--release` build to bake this data into a fast binary.

## 4. Why this matters for Master
By having the **Jieqi crossover times** baked into the data:
- We eliminate "boundary errors" in Bazi charts (e.g., someone born on the day of Lichun).
- We can automate the generation of cases in our library with 100% structural correctness.

---
*Created: 2026-02-09*
*Status: Draft / Awaiting Master's feedback on specific data fields.*
