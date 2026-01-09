# jc-zed-tasks

Here is a rust binary project that consolidate many of my Zed Tasks I am using. 

Related Repos: 
- [jc-zed-config](https://github.com/jeremychone/jc-zed-config)
- [jc-zed-tasks (this one)](https://github.com/jeremychone/jc-zed-tasks)
- [jc-tmux-config](https://github.com/jeremychone/jc-tmux-config)
- [jc-alacritty-config](https://github.com/jeremychone/jc-alacritty-config)

Feel free to use it as such, or cherry pick what you need, or fork and make it your own.

NOTE: It is being developed and tested on a Mac. PRs for Linux are welcome.

## Usage

```sh
# Toggle Zed AI on/off
jc-zed-tasks zed-toggle-ai

# Save clipboard image to a directory (auto-incremented)
jc-zed-tasks save-clipboard-image --dir ./images

# Save current md file into html
jc-zed-tasks md-to-html --file path/to/file.md

# Send 'r' to first TMUX pane with this dir
jc-zed-tasks tmux-run-aip --dir .

# Position Alacritty term at below this zed term
jc-zed-tasks new-dev-term --cwd . --pos below
```


### `zed-toggle-ai`

Toggle AI features in Zed settings (`~/.config/zed/settings.json`).

```sh
jc-zed-tasks zed-toggle-ai
```

### `save-clipboard-image`

Save the current image from the clipboard to a directory. 
It looks for `image-*.png` files and uses the next available number (e.g., `image-01.png`, `image-02.png`).

```sh
jc-zed-tasks save-clipboard-image --dir ./docs/images
```

### `md-to-html`

Convert a Markdown file to HTML.

```sh
jc-zed-tasks md-to-html --file path/to/file.md
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
- `--show-if-present`: If terminal with same title exists, show it instead of creating a new one.

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
