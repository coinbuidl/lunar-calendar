use crate::data::BAZI_YEAR_DATA;
use crate::table_lookup::{
    BRANCHES, MAX_SUPPORTED_YEAR, MIN_SUPPORTED_YEAR, STEMS, get_cycle_index,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use std::sync::OnceLock;

/// Pillars of a specific date and time.
#[derive(Debug)]
pub struct BaziPillars {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
}

impl std::fmt::Display for BaziPillars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}年 {}月 {}日 {}时",
            self.year, self.month, self.day, self.hour
        )
    }
}

/// Information about a specific time point relative to solar terms.
#[derive(Debug)]
pub struct JieQiContext {
    pub prev_name: &'static str,
    pub prev_time: NaiveDateTime,
    pub next_name: &'static str,
    pub next_time: NaiveDateTime,
    pub diff_prev_seconds: i64,
    pub diff_next_seconds: i64,
}

impl std::fmt::Display for JieQiContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_days = self.diff_prev_seconds / 86400;
        let prev_hours = (self.diff_prev_seconds % 86400) / 3600;
        let next_days = self.diff_next_seconds / 86400;
        let next_hours = (self.diff_next_seconds % 86400) / 3600;
        write!(
            f,
            "前气: {} ({}, 距今 {}天{}小时), 后气: {} ({}, 距今 {}天{}小时)",
            self.prev_name,
            self.prev_time,
            prev_days,
            prev_hours,
            self.next_name,
            self.next_time,
            next_days,
            next_hours
        )
    }
}

#[derive(Clone)]
struct ParsedJieQi {
    name: &'static str,
    time: NaiveDateTime,
}

#[derive(Clone)]
struct ParsedYearData {
    year: i32,
    lichun_time: NaiveDateTime,
    jieqi: [ParsedJieQi; 24],
}

static PARSED_YEAR_TABLE: OnceLock<Option<Vec<ParsedYearData>>> = OnceLock::new();

fn parse_ts(value: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").ok()
}

fn parsed_year_table() -> Option<&'static [ParsedYearData]> {
    PARSED_YEAR_TABLE
        .get_or_init(|| {
            let mut parsed = Vec::with_capacity(BAZI_YEAR_DATA.len());
            for row in BAZI_YEAR_DATA.iter() {
                let lichun_time = parse_ts(row.lichun_time)?;

                let mut jieqi = Vec::with_capacity(row.jieqi.len());
                for jq in row.jieqi.iter() {
                    let time = parse_ts(jq.time)?;
                    jieqi.push(ParsedJieQi {
                        name: jq.name,
                        time,
                    });
                }

                let jieqi: [ParsedJieQi; 24] = jieqi.try_into().ok()?;
                parsed.push(ParsedYearData {
                    year: row.year,
                    lichun_time,
                    jieqi,
                });
            }
            Some(parsed)
        })
        .as_deref()
}

fn parsed_year_data(year: i32) -> Option<&'static ParsedYearData> {
    if !(MIN_SUPPORTED_YEAR..=MAX_SUPPORTED_YEAR).contains(&year) {
        return None;
    }
    let idx = (year - MIN_SUPPORTED_YEAR) as usize;
    let row = parsed_year_table()?.get(idx)?;
    debug_assert_eq!(row.year, year);
    Some(row)
}

/// Helper to get formatted stem-branch string.
fn format_gz(index: usize) -> String {
    let idx = index % 60;
    format!("{}{}", STEMS[idx % 10], BRANCHES[idx % 12])
}

fn normalize_jie_name(name: &'static str) -> &'static str {
    if name == "DA_XUE" { "大雪" } else { name }
}

fn month_index_from_jie(name: &str) -> Option<usize> {
    match name {
        "立春" => Some(0),
        "惊蛰" => Some(1),
        "清明" => Some(2),
        "立夏" => Some(3),
        "芒种" => Some(4),
        "小暑" => Some(5),
        "立秋" => Some(6),
        "白露" => Some(7),
        "寒露" => Some(8),
        "立冬" => Some(9),
        "大雪" => Some(10),
        "小寒" => Some(11),
        _ => None,
    }
}

/// Returns the surrounding JieQi (Solar Terms) for a given timestamp.
pub fn get_jieqi_context(target_time: NaiveDateTime) -> Option<JieQiContext> {
    let year = target_time.date().year();
    let mut prev: Option<(&'static str, NaiveDateTime)> = None;
    let mut next: Option<(&'static str, NaiveDateTime)> = None;

    for y in [year - 1, year, year + 1] {
        if let Some(data) = parsed_year_data(y) {
            for term in &data.jieqi {
                if term.time <= target_time {
                    if prev.as_ref().map(|(_, t)| term.time > *t).unwrap_or(true) {
                        prev = Some((term.name, term.time));
                    }
                } else if next.as_ref().map(|(_, t)| term.time < *t).unwrap_or(true) {
                    next = Some((term.name, term.time));
                }
            }
        }
    }

    let (prev_name, prev_time) = prev?;
    let (next_name, next_time) = next?;

    Some(JieQiContext {
        prev_name,
        prev_time,
        next_name,
        next_time,
        diff_prev_seconds: (target_time - prev_time).num_seconds(),
        diff_next_seconds: (next_time - target_time).num_seconds(),
    })
}

/// Calculate the pillars for a given date and time.
pub fn get_pillars(dt: NaiveDateTime) -> Option<BaziPillars> {
    let year_data = parsed_year_data(dt.year())?;

    // 1. Year pillar based on LiChun boundary.
    let bazi_year = if dt < year_data.lichun_time {
        dt.year() - 1
    } else {
        dt.year()
    };
    let year_gz = format_gz(get_cycle_index(bazi_year));

    // 2. Month pillar based on latest "Jie" before the input.
    let mut current_jie: Option<(&'static str, NaiveDateTime)> = None;
    for y in [dt.year() - 1, dt.year(), dt.year() + 1] {
        if let Some(data) = parsed_year_data(y) {
            for term in &data.jieqi {
                let name = normalize_jie_name(term.name);
                if month_index_from_jie(name).is_some() && term.time <= dt {
                    if current_jie
                        .as_ref()
                        .map(|(_, t)| term.time > *t)
                        .unwrap_or(true)
                    {
                        current_jie = Some((name, term.time));
                    }
                }
            }
        }
    }
    let month_idx = month_index_from_jie(current_jie?.0)?;

    // Month Stem: (Year Stem index * 2 + Month index + 2) % 10
    let year_stem_idx = (get_cycle_index(bazi_year) % 10) as i32;
    let m_stem_idx = (year_stem_idx * 2 + (month_idx as i32) + 2) % 10;
    let m_branch_idx = (month_idx + 2) % 12; // Yin = 2
    let month_gz = format!("{}{}", STEMS[m_stem_idx as usize], BRANCHES[m_branch_idx]);

    // 3. Day pillar. Base date 1900-01-01 is 甲戌 (index 10).
    let base_date = NaiveDate::from_ymd_opt(1900, 1, 1)?;
    let diff_days = (dt.date() - base_date).num_days();
    let day_idx = (10 + diff_days).rem_euclid(60) as usize;
    let day_gz = format_gz(day_idx);

    // 4. Hour pillar.
    let h_branch_idx = ((dt.hour() + 1) / 2) % 12;
    let day_stem_idx = (day_idx % 10) as i32;
    let h_stem_idx = (day_stem_idx * 2 + (h_branch_idx as i32)) % 10;
    let hour_gz = format!(
        "{}{}",
        STEMS[h_stem_idx as usize], BRANCHES[h_branch_idx as usize]
    );

    Some(BaziPillars {
        year: year_gz,
        month: month_gz,
        day: day_gz,
        hour: hour_gz,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_dt(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn known_pillars_case() {
        let dt = parse_dt("1985-04-21 08:00:00");
        let pillars = get_pillars(dt).unwrap();
        assert_eq!(pillars.to_string(), "乙丑年 庚辰月 庚寅日 庚辰时");
    }

    #[test]
    fn known_jieqi_context_case() {
        let dt = parse_dt("1985-04-21 08:00:00");
        let ctx = get_jieqi_context(dt).unwrap();
        assert_eq!(ctx.prev_name, "谷雨");
        assert_eq!(ctx.next_name, "立夏");
    }

    #[test]
    fn regression_user_verified_pillars() {
        let cases = [
            ("2024-01-28 12:00:00", "癸卯年 乙丑月 辛卯日 甲午时"),
            ("1992-01-08 08:00:00", "辛未年 辛丑月 癸未日 丙辰时"),
            ("1992-01-02 08:00:00", "辛未年 庚子月 丁丑日 甲辰时"),
            ("1958-12-10 00:00:00", "戊戌年 甲子月 辛酉日 戊子时"),
            ("1996-11-22 12:00:00", "丙子年 己亥月 癸亥日 戊午时"),
            ("1990-12-30 08:00:00", "庚午年 戊子月 己巳日 戊辰时"),
        ];

        for (input, expected) in cases {
            let dt = parse_dt(input);
            let got = get_pillars(dt).unwrap().to_string();
            assert_eq!(got, expected, "failed for input {input}");
        }
    }

    #[test]
    fn regression_december_daxue_switches_to_zi_month() {
        let dt = parse_dt("1958-12-10 00:00:00");
        let ctx = get_jieqi_context(dt).unwrap();
        let pillars = get_pillars(dt).unwrap();
        assert_eq!(ctx.prev_name, "大雪");
        assert_eq!(pillars.month, "甲子");
    }
}
