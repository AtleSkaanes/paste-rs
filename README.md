# Paste-rs

A simple CLI tool for interacting with the [paste.rs](https://paste.rs/) API.

## Functionality

#### Send

Use the `send` command to send some Data to [paste.rs](https://paste.rs/).

Just put a string on to send that to the servers.

```bash
paste-rs send "HELLO WORLD"
```

Or put a file path, with the `-f` or `--files` flag, to send the contents of a file.

```bash
paste-rs send ./file.txt -f
```

You can also use piping to send the result from another CLI.

```bash
echo "HELLO WORLD" | paste-rs send
```

#### Get

Use the `get` command to get data from the [paste.rs](https://paste.rs/) servers.
You can either put the data's `id`, or the URL to the [paste.rs](https://paste.rs/) site of the data.

```bash
paste-rs get [SOME ID]
```

or

```bash
paste-rs get https://paste.rs/[SOME ID]
```

If you want the contents of the data to be put into STDOUT, for either quickly viewing it in the terminal, or for piping it to some other program, just give the data `id` or `url`.
But if you want it to be saved in a file, you can supply it with an output path, via the `-o` or `--output` argument.

If you want to get a HTML file, with rendered Markdown, or syntax highlighting for code, you can use the `-e` or `--extension` argument, to set what kind of file you want rendered, like `rs` for rust files, `cpp` for C++ files, or `md` for markdown files.

```bash
paste-rs get [SOME ID] -o ./file.txt
```

This will make a new file called `file.txt`, and put the data inside it.

The `id` can also be piped from another program.

#### Delete

To not fill the [paste.rs](https://paste.rs/) servers, with data no longer used. You can delete data you don't need anymore, by using the `delete` command, along with the `id` or `url` to the data you want to delete.

```bash
paste-rs delete [SOME ID]
```

The `id` can also be piped from another program.

#### Open

To open some data already on the [paste.rs](https://paste.rs/) servers, in your browser, you can use the `open` command, along with the datas `id` or `url`.

```bash
paste-rs open [SOME ID]
```

To get markdown files rendered, or code files with syntax highlight, you can also use the `-e` or `--extension` argument, to set what kind of file it should be rendered as.

The `id` can also be piped from another program.
