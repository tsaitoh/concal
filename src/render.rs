use crate::calendar::CalendarGrid;
use atty::Stream;
use chrono::Datelike;

#[derive(Debug, Clone, Copy, clap::ValueEnum, Default)]
pub enum ColorChoice {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    Color,
    NoColor,
}

impl From<ColorChoice> for ColorMode {
    fn from(choice: ColorChoice) -> Self {
        match choice {
            ColorChoice::Always => ColorMode::Color,
            ColorChoice::Never => ColorMode::NoColor,
            ColorChoice::Auto => {
                if atty::is(Stream::Stdout) {
                    ColorMode::Color
                } else {
                    ColorMode::NoColor
                }
            }
        }
    }
}

const LEVEL_0_PLAIN: &str = "░";
const LEVEL_1_PLAIN: &str = "▒";
const LEVEL_2_PLAIN: &str = "▓";
const LEVEL_3_PLAIN: &str = "█";
const LEVEL_4_PLAIN: &str = "█";

const LEVEL_0_COLOR: u8 = 235;
const LEVEL_1_COLOR: u8 = 40;
const LEVEL_2_COLOR: u8 = 46;
const LEVEL_3_COLOR: u8 = 82;
const LEVEL_4_COLOR: u8 = 118;

fn get_level_char(count: u32, mode: ColorMode) -> String {
    match mode {
        ColorMode::NoColor => match count {
            0 => LEVEL_0_PLAIN.to_string(),
            1..=2 => LEVEL_1_PLAIN.to_string(),
            3..=5 => LEVEL_2_PLAIN.to_string(),
            6..=9 => LEVEL_3_PLAIN.to_string(),
            _ => LEVEL_4_PLAIN.to_string(),
        },
        ColorMode::Color => {
            let color = match count {
                0 => LEVEL_0_COLOR,
                1..=2 => LEVEL_1_COLOR,
                3..=5 => LEVEL_2_COLOR,
                6..=9 => LEVEL_3_COLOR,
                _ => LEVEL_4_COLOR,
            };
            format!("\x1b[38;5;{}m■\x1b[0m", color)
        }
    }
}

pub fn render(grid: &CalendarGrid, mode: ColorMode) -> String {
    if grid.is_empty() || grid[0].is_empty() {
        return "No data to display".to_string();
    }
    let weeks = grid[0].len();
    let mut s = String::new();
    s.push_str("    ");
    let mut w = 0;
    while w < weeks {
        let month = grid[0][w].date.month();
        let prev_month = if w > 0 { grid[0][w - 1].date.month() } else { 0 };

        if w == 0 || month != prev_month {
            if w + 1 < weeks { 
                let label = grid[0][w].date.format("%b").to_string();
                s.push_str(&label);
                s.push(' ');
                w += 2;
            } else {
                s.push_str("  ");
                w += 1;
            }
        } else {
            s.push_str("  ");
            w += 1;
        }
    }
    s.push('\n');
    let week_days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for d in 0..7 {
        if d % 2 != 0 {
            s.push_str(&format!("{:<4}", week_days[d]));
        } else {
            s.push_str("    ");
        }
        for w in 0..weeks {
            let cell = &grid[d][w];
            s.push_str(&get_level_char(cell.count, mode));
            s.push_str(" ");
        }
        s.push('\n');
    }
    s
}