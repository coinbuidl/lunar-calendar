#[path = "lunar-calendar-data-1900-2099.rs"]
pub mod data;

mod bazi_calc;
mod table_lookup;

pub use bazi_calc::{BaziPillars, JieQiContext, get_jieqi_context, get_pillars};
pub use table_lookup::{
    MAX_SUPPORTED_YEAR, MIN_SUPPORTED_YEAR, get_cycle_index, get_nayin_by_index, get_nayin_by_year,
    get_stem_branch, get_year_data,
};
