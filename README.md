# fuzzymatch

A WASM library for fuzzy matching strings.

# How to build

Run build.bat or just run the two commands manually.  This will build a node and a browser version in `crate/dist/pkg-web` and `crate/dist/pkg-node` respectively.

# How to run in web

Use an http server after building like `npx http-server -c-1 .` and open up `example/web`'s index.html.

# How to run in node

run `node index.js "[searchstring]"` in `example/node`'s directory after building.