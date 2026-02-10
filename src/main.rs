use chrono::NaiveDateTime;
use lunar_calendar::*;

fn parse_builtin_datetime(input: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S").ok()
}

fn main() {
    println!("--- 60甲子纳音五行对照表 ---");
    println!("{:<4} {:<4} {:<8}", "序号", "干支", "纳音五行");

    for i in 0..60 {
        if let (Some((stem, branch)), Some(nayin)) = (get_stem_branch(i), get_nayin_by_index(i)) {
            println!("{:<4} {}{} {:<8}", i + 1, stem, branch, nayin);
        }
    }

    println!("\n--- 示例查询 (算法计算) ---");
    let test_years = [1984, 1985, 2024, 2026];
    for &year in &test_years {
        let index = get_cycle_index(year);
        if let Some((s, b)) = get_stem_branch(index) {
            println!("{}年 ({}{}): {}", year, s, b, get_nayin_by_year(year));
        }
    }

    println!("\n--- 年数据查询 (BAZI_YEAR_DATA) ---");
    let db_years = [1900, 2024, 2099];
    for &year in &db_years {
        if let Some(info) = get_year_data(year) {
            println!(
                "{}年: 农历新年={}, 立春={}",
                info.year, info.lunar_new_year, info.lichun_time
            );
        }
    }

    println!("\n--- 八字查询 (Bazi Pillars) ---");
    // Test with Case 06: Geng Zi, Xin Si, Jia Wu, Jia Xu
    // 1960-05-18 20:00:00
    let Some(test_dt) = parse_builtin_datetime("1960-05-18 20:00:00") else {
        eprintln!("Failed to parse built-in datetime: 1960-05-18 20:00:00");
        return;
    };
    if let Some(pillars) = get_pillars(test_dt) {
        println!("时间: {}", test_dt);
        println!("八字: {}", pillars);
    }

    // Test with current time (approx)
    let Some(now_dt) = parse_builtin_datetime("2026-02-09 18:00:00") else {
        eprintln!("Failed to parse built-in datetime: 2026-02-09 18:00:00");
        return;
    };
    if let Some(pillars) = get_pillars(now_dt) {
        println!("时间: {}", now_dt);
        println!("八字: {}", pillars);
    }

    println!("\n--- Master Inquiry (21 Feb 1985 08:00) ---");
    let Some(master_dt) = parse_builtin_datetime("1985-02-21 08:00:00") else {
        eprintln!("Failed to parse built-in datetime: 1985-02-21 08:00:00");
        return;
    };
    if let Some(pillars) = get_pillars(master_dt) {
        println!("时间: {}", master_dt);
        println!("八字: {}", pillars);
        if let Some(ctx) = get_jieqi_context(master_dt) {
            println!("{}", ctx);
        }
    }

    println!("\n--- Master Inquiry (21 Apr 1985 08:00) ---");
    let Some(master_dt2) = parse_builtin_datetime("1985-04-21 08:00:00") else {
        eprintln!("Failed to parse built-in datetime: 1985-04-21 08:00:00");
        return;
    };
    if let Some(pillars) = get_pillars(master_dt2) {
        println!("时间: {}", master_dt2);
        println!("八字: {}", pillars);
        if let Some(ctx) = get_jieqi_context(master_dt2) {
            println!("{}", ctx);
        }
    }

    println!("\n--- 节气距离引擎 (JieQi Distance Engine) ---");
    if let Some(ctx) = get_jieqi_context(test_dt) {
        println!("查询时间: {}", test_dt);
        println!("{}", ctx);
    }
}
