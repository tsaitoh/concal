# Contribution Calendar

A CLI tool to display a GitHub-style contribution calendar in the terminal.
This tool analyzes the commit history of a Git repository and visualizes the contribution activity over the last year.

## Build

To build the project, use Cargo:

```sh
cargo build --release
```

The executable will be available at `target/release/concal`.

## Usage

Run the executable with the path to a Git repository.

```sh
./target/release/concal [PATH_TO_REPO]
```

### Arguments

*   `PATH`: (Optional) The path to the root of the Git repository you want to analyze. If omitted, it defaults to the current directory.
*   `--weeks <WEEKS>`: (Optional) The number of weeks of commit history to display. Defaults to 52.
*   `--color <COLOR>`: (Optional) Specifies when to use ANSI colors in the output.
    *   `auto`: (Default) Use colors only if the output is a TTY.
    *   `always`: Always use colors.
    *   `never`: Never use colors.

### Example

```sh
# Analyze the repository in the current directory
./target/release/concal

# Analyze a different repository
./target/release/concal ~/projects/my-repo

# Display 26 weeks with colors always enabled
./target/release/concal --weeks 26 --color always

# Consider activity from all branches
./target/release/concal --all
```
