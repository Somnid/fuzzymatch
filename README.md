# fuzzymatch

A WASM library for matching strings fuzzily.

# How to build

Run build.bat or just run the two commands manually.  This will build a node and a browser version in `pkg-web` and `pkg-node` respectively.

# How to run in web

Use an http server after building like `npx http-server -c-1 .` and open up example-web's index.html.

# How to run in node

run `node index.js "[searchstring]"` in example-node's directory after building.