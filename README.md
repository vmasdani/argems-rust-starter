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
To build, you need Python 3, and also [Node.JS](https://nodejs.org/en/download/) for minification.  
Edit your app URL in `release.json`.
```
sudo npm i -g uglify-js
./release.py
```
