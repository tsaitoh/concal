use std::collections::HashMap;

use chrono::{Datelike, Duration, Local, NaiveDate};

#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub date: NaiveDate,
    pub count: u32,
}

pub type CalendarGrid = Vec<Vec<Cell>>;

/// 日次コミット数のマップからコントリビューションカレンダー用のグリッドを生成する。
pub fn build_grid(daily_counts: &HashMap<NaiveDate, u32>, weeks: u32) -> CalendarGrid {
    let end_date = Local::now().date_naive();
    let start_date = end_date - Duration::weeks(weeks as i64 - 1);
    let start_date = start_date - Duration::days(start_date.weekday().num_days_from_sunday() as i64);

    let mut grid = vec![vec![Cell::default(); weeks as usize]; 7];
    let current_date = start_date;

    for w in 0..weeks {
        for d in 0..7 {
            let date = current_date + Duration::days(d as i64 + w as i64 * 7);
            grid[d as usize][w as usize] = Cell {
                date,
                count: *daily_counts.get(&date).unwrap_or(&0),
            };
        }
    }

    grid
}