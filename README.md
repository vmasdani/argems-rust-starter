# ARGEMS Rust Starter

This is a starter Rust ARGEMS code which has REST API and database interaction.

## .env file
Create .env file containing:
```
DATABASE_URL=argems.sqlite3
```

## Development

- Install prerequisites
    1. [Rust](https://www.rust-lang.org/tools/install)
    2. [Elm](https://guide.elm-lang.org/install/elm.html)
    3. If running ubuntu based distro, you might need to install some dependencies
```sudo apt install build-essential libsqlite3-dev```

- Rust
```
cargo run
```

- Elm
```
cd frontend
./run.sh
```

# Build
- To build, you need Python 3, and also [Node.JS](https://nodejs.org/en/download/) for minification.  

- Add `env.json` file consisting:
```
{
  "base_url": "http://localhost:8080",
  "server_port": "8080"
} 
```

- Install [Docker](https://docs.docker.com/get-docker/) and [cross](https://github.com/rust-embedded/cross)  

- Then run:
```
sudo npm i -g uglify-js
./release.py
```
