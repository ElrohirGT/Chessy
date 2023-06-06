# Chessy

A toy chess app written in rust and {insert javascript framework here}.

## Roadmap
[The roadmap can be seen on trello](https://trello.com/invite/b/ZL1dRXbG/ATTIec8041a0c595db27db355cca3a4725e139F9AEF9/chessy)

## Compiling the project
All developer dependencies are defined on the shell.nix file. To use this file just install Nix and then proceed with the command:
```bash
nix-shell
``` 

All terminal command blocks assume your on the base directory of this repo and that you are inside the developer session that the command above creates.

This command will read the nix file and setup everything that you'll need to develop the app. Everytime you wish to compile the app you'll need to enter this command to have access to all the dev-tools.

The first time you execute the command it may take a while because it needs to install node and rust.

### Compiling the Chess Engine
Run the following commands to compile the Chess Engine written in rust:
```bash
cd chess_engine
cargo build --lib
```

You can also compile the Chess Engine to be used on web assembly by running:
```bash
cd chess_engine
wasm-pack pack --target web
```
This generates a pkg directory inside the `chess_engine` directory. This folder represents the module to be used inside the javascript client. [For more information you can watch this video](https://www.youtube.com/watch?v=nW71Mlbmxt8)
