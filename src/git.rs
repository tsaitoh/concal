use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use chrono::{Duration, Local, NaiveDate};

/// 指定したリポジトリから指定週数分のコミット数を日毎に集計する。
pub fn collect_daily_counts(repo_path: &Path, weeks: u32) -> Result<HashMap<NaiveDate, u32>> {
    let since = Local::now().naive_local() - Duration::weeks(weeks as i64 - 1);
    let since = since.date().and_hms_opt(0, 0, 0).unwrap();

    let output = Command::new("git")
        .arg("log")
        .arg("--date=iso")
        .arg(format!("--since={}", since.format("%Y-%m-%d")))
        .arg("--format=%cd")
        .current_dir(repo_path)
        .output()
        .context("git logの実行に失敗しました")?;

    if !output.status.success() {
        anyhow::bail!(
            "git logの実行に失敗しました: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut counts = HashMap::new();
    for line in stdout.lines() {
        if let Ok(date) = NaiveDate::parse_from_str(line.split_whitespace().next().unwrap(), "%Y-%m-%d") {
            *counts.entry(date).or_insert(0) += 1;
        }
    }

    Ok(counts)
}
