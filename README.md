# jc-zed-tasks

Here is a rust binary project that consolidate many of my Zed Tasks I am using.

Feel free to use it as such, or cherry pick what you need, or fork and make it your own.

A collection of CLI tools for AIP development workflows, including tmux session management and project initialization.

Related: 
- [jc-zed-config](https://github.com/jeremychone/jc-zed-config)
- [jc-zed-tasks (this one)](https://github.com/jeremychone/jc-zed-tasks)

## Usage

### `zed-toggle-ai`

Toggle AI features in Zed settings (`~/.config/zed/settings.json`).

```sh
jc-zed-tasks zed-toggle-ai
```

### `tmux-run-aip`

Send a 'r' to the first active 'aip' (AIPack) tmux pane that run in this dir. 

- `--dir <PATH>`: Filter by pane directory (required).
- `--pane <NAME>`: Filter by pane name/title (optional).

```sh
# To run the first 
jc-zed-tasks tmux-run-aip --dir /some/path/to/dir

# To run a specicif AIPack pane with agent name (which AIPack set at start)
jc-zed-tasks tmux-run-aip --dir /some/path/to/dir --pane pro@coder
```

### `new-dev-term`

Open a new Alacritty development terminal.

- `--cwd <PATH>`: Working directory for the new terminal.
- `--with-tmux`: Start tmux in the new terminal.
- `--pos <below|bottom>`: Position the terminal relative to Zed.

```sh
jc-zed-tasks new-dev-term --cwd . --pos bottom
```

## Development


```sh

# Build & Install
cargo install --path .

# Watch (great for in dev)
cargo watch -c -x "install --path ."

# Build and Help
# Build the project
cargo build --release
```



[This repo](https://github.com/jeremychone/jc-zed-tasks)
