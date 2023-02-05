# Rusty ECMA Benches

This project is used to generate benchmarks for using both RESS and RESSA to
parse some popular JS libraries.

## build.rs

The build.rs file here uses the handlebars templating engine to generate a bunch of files
in the `src/bin` directory. Nearly all of the JS libraries used here provide both a "normal"
and minified version, some also provide an es modules version, two binaries is generated for
each of these provided files one for benching ress and one for benching ressa, that is up to 6
benchmarks per JS library.

The primary reason these are created as independent binaries is to utilize the `include_str!` macro
as apposed to using any kind of IO to read the javascript. This way the benchmarks are more about
the time it takes to parse each file.

## scripts

### check_tags.js

This file is setup to query the current tags for both ress and ressa, if those have been added to
the cargo.toml file here is updated to point to the most recent tag.

### compile_all.sh

This file iterates through the files created in `src/bin` and compiles each binary with the
`--release` flag and then pulls that out of the `target/release` folder and puts it in `/bins`
this is really just to make things easier for the script `run_all.sh`.

### post_run.js

This script will iterate through all of the results in a single run and insert them into a sqlite
database.

### run_all.sh

This actually performs the benchmarks for each file in `bins` using `hyperfine` placing the
results into the folder `results/<result-date>[-run-number]` and then executes the `post_run.js`
script.

## Benchmarks

| JS Lib               | Normal | Min | Module |
| ------               | ------ | --- | ------ |
| angular@1            | ✅     | ✅  |  🚫    |
| dexie                | ✅     | ✅  |  🐞    |
| everything.js es5    | ✅     | 🚫  |  🚫    |
| everything.js es2015 | ✅     | 🚫  |  ✅    |
| jquery               | ✅     | ✅  |  🚫    |
| moment               | ✅     | ✅  |  ✅    |
| react                | ✅     | ✅  |  🚫    |
| react-dom            | ✅     | ✅  |  🚫    |
| vue                  | 🐞     | ✅  |  ✅    |

> 🚫: file not provided
>
> ✅ file provided
>
> 🐞: file provided but a bug currently prevents it from being benchmarked
