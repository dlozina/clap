#!/usr/bin/env node

const cli = require("./index")
const arguments = process.argv.slice(2)

// Log all the arguments received
console.log("All arguments:", process.argv);

// Skip the first two arguments and get the path and pattern
const [path, pattern] = process.argv.slice(2);

if (!path || !pattern) {
    console.error("Usage: node <PATH> <PATTERN>");
    process.exit(1);
}

// Pass the path and pattern as arguments to cli.run()
cli.run(path, pattern).catch((e) => {
    console.error(e);
    process.exit(1);
});