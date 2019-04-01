const { fuzzymatch } = require("../pkg-node/fuzzymatch.js");
const titles = require("./titles.js");

const term = process.argv[2];

console.log(`search term: ${term}`);

console.log(fuzzymatch(titles, term, 0.5).map(x => x[1]));