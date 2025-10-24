mod calendar;
mod git;
mod render;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// GitHub風コントリビューションカレンダーをターミナルに描画するCLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// リポジトリルート（省略時はカレントディレクトリ）
    #[arg(value_name = "PATH", default_value = ".")]
    repo: PathBuf,

    /// 集計する週数
    #[arg(long, default_value_t = 52)]
    weeks: u32,

    /// ANSIカラーの扱い
    #[arg(long, default_value_t = render::ColorChoice::Auto, value_enum)]
    color: render::ColorChoice,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let repo_path = cli.repo.canonicalize()?;
    let daily = git::collect_daily_counts(&repo_path, cli.weeks)?;
    let grid = calendar::build_grid(&daily, cli.weeks);
    let rendered = render::render(&grid, cli.color.into());

    println!("{}", rendered);
    Ok(())
}
