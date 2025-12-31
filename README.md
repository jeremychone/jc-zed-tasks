# zed-tasks

A collection of CLI tools for AIP development workflows, including tmux session management and project initialization.

## Usage

### `tmux-run-aip`

Run AIP in a tmux session (sends the 'r' key to the target pane).

- `--dir <PATH>`: Filter by pane directory (required).
- `--pane <NAME>`: Filter by pane name/title (optional).

```sh
zed-tasks tmux-run-aip --dir . --pane pro@coder
```

### `create-git-ignore`

Create a `.gitignore` file at the specified path.

- `<PATH>`: Target directory path.

```sh
zed-tasks create-git-ignore .
```

### `zed-toggle-ai`

Toggle AI features in Zed settings (`~/.config/zed/settings.json`).

```sh
zed-tasks zed-toggle-ai
```

### `new-dev-term`

Open a new Alacritty development terminal.

- `--cwd <PATH>`: Working directory for the new terminal.
- `--with-tmux`: Start tmux in the new terminal.
- `--pos <below|bottom>`: Position the terminal relative to Zed.

```sh
zed-tasks new-dev-term --cwd . --pos bottom
```

## Development


```sh

# -- Build and Help
# Build the project
cargo build --release

# View all available commands and options
zed-tasks --help
```
