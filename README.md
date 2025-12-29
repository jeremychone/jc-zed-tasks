# zed-tasks

A collection of CLI tools for AIP development workflows, including tmux session management and project initialization.

## Usage

```sh
# -- tmux-run-aip: Lists all current tmux sessions, windows, and panes.
# Run the tmux session lister
zed-tasks tmux-run-aip --dir /Users/jeremychone/_jeremy/_rust/utils/rust-zed-tasks --pane-name pro@coder

# -- create-git-ignore: Generates a .gitignore file at the specified target path.
# Usage: zed-tasks create-git-ignore <path>
zed-tasks create-git-ignore .

# -- new-dev-term: Opens a new Alacritty terminal in the specified directory.
# Usage: zed-tasks new-dev-term --cwd <path>
zed-tasks new-dev-term --cwd .

```

## Development


```sh

# -- Build and Help
# Build the project
cargo build --release

# View all available commands and options
zed-tasks --help
```
