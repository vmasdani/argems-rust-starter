#!/usr/bin/env python3

import subprocess
import sys
import json

# Read release.json
release = open('release.json')
release_contents = json.loads(release.read())

release.close()

base_url = release_contents["base_url"]

index_html_contents = """<html>
    <head>
    <meta charset="UTF-8">
    <title>Main</title>
    <script src="main.js"></script>
    <script src="script.js"></script>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"> 
    </head>

    <body>
    <div id="myapp"></div>
    <script>
    var app = Elm.Main.init({{
        node: document.getElementById('myapp'),
        flags: '{}'
    }});
    </script>
    </body>
</html>""".format(base_url)

steps = [
    ("mkdir -p dist/frontend", "."),
    ("cargo build --release", "."),
    ("cp target/release/argems-rust-starter .env dist", "."),
    ("./build.sh", "./frontend"),
    ("cp dist/* ../dist/frontend", "./frontend"),
]

for (cmd, cwd) in steps:
    subprocess.run(cmd, shell=True, cwd=cwd)

f = open('./dist/frontend/index.html', 'w+')
f.write(index_html_contents)
f.close()

print("\nRelease success! files are located in ./dist")
