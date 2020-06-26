<div align="center">
  <a href="https://github.com/rousan/maui">
    <img width="200" height="200" src="https://avatars3.githubusercontent.com/u/63495711?s=200&v=4">
  </a>
  <br />
  <br />
</div>

[![Crate](https://img.shields.io/crates/v/maui.svg)](https://crates.io/crates/maui)
[![Contributors](https://img.shields.io/github/contributors/rousan/maui.svg)](https://github.com/rousan/maui/graphs/contributors)
[![MIT](https://img.shields.io/crates/l/maui.svg)](./LICENSE)

[WIP]

# maui

A fast TCP connection tunneling tool, useful for any TCP protocol http, https, ftp, websocket, mysql etc.

## What is it?

blah....

## How to use it?

blah....

## Install CLI

### macOS

```sh
 $ bash -c "$(curl -fsSL https://raw.githubusercontent.com/rousan/maui/master/install.sh)"
```

### Linux

```sh
 $ bash -c "$(curl -fsSL https://raw.githubusercontent.com/rousan/maui/master/install.sh)"
```

### Windows

Please download it from [releases](https://github.com/rousan/maui/releases) page.

## Development

### Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

### Commands

Setup the project by installing all the required dev tools:

```sh
tusk setup
```

Start the server in dev mode:

```sh
tusk server:start:dev
```

Start the server in prod mode:

```sh
tusk server:start:prod
```

Run the CLI in dev mode:

```sh
tusk cli:run:dev
```

Please refer to `tusk.yml` file for more commands.

### Deploy the server in self-hosted mode

## Contributing

Your PRs and suggestions are always welcome.
