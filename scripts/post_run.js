const fs = require("fs").promises;
const path = require("path");
const toml = require("toml");
const csv = require("csv");
const sqlite3 = require("sqlite3")
const sqlite = require("sqlite");

async function parse_csv(contents) {
    return await new Promise((r, j) => {
        csv.parse(contents, {
            columns: true,
            group_columns_by_name: true,
        }, (err, recs) => {
            if (err) {
                return j(err);
            }
            return r(recs);
        })
    })
}

function transform_results(results) {
    for (let result of results) {
        let re_matches = result.command.match(/bins\/(ressa?)_(.+)/);
        delete result.command;
        result.project = re_matches[1];
        result.lib = re_matches[2];
        result.mean = +result.mean
        result.stddev = +result.stddev
        result.median = +result.median
        result.user = +result.user
        result.system = +result.system
        result.min = +result.min
        result.max = +result.max
    }
    return results;
}

async function stringify_csv(t) {
    return await new Promise((r, j) => {
        csv.stringify(t, { header: true }, (err, text) => {
            if (err) {
                return j(err)
            }
            return r(text)
        })
    });
}

async function ensure_tables(db) {
    let rows = await db.all("SELECT name FROM sqlite_master WHERE type='table'");
    let tables = new Set(rows.map(o => o.name));
    if (!tables.has("runs")) {
        await db.run(`CREATE TABLE "runs" (
            "id" TEXT NOT NULL UNIQUE,
            "ressa_version" TEXT NOT NULL,
            "ress_version" TEXT NOT NULL,
            PRIMARY KEY("id")
        );`);
    }
    if (!tables.has("benches")) {
        await db.run(`CREATE TABLE benches (
            run_id TEXT NOT NULL,
            lib TEXT NOT NULL,
            project TEXT NOT NULL,
            mean REAL NOT NULL,
            stddev REAL NOT NULL,
            median REAL NOT NULL,
            user REAL NOT NULL,
            system REAL NOT NULL,
            min REAL NOT NULL,
            max REAL NOT NULL,
	        FOREIGN KEY("run_id") REFERENCES "runs"("id"),
            PRIMARY KEY("run_id","project","lib")
        );`)
    }
}

async function write_run(results, id, ressa_version, ress_version) {
    let db = await sqlite.open({
        filename: "results.sqlite",
        driver: sqlite3.Database,
    });
    await ensure_tables(db);
    console.log("inserting run", id, ressa_version, ress_version);
    await db.run("INSERT INTO runs (id, ressa_version, ress_version) VALUES (?, ?, ?);", id, ressa_version, ress_version);
    console.log("preparing statement");
    let stmt = await db.prepare("INSERT INTO benches (run_id, lib, project, mean, stddev, median, user, system, min, max) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);")
    for (let bench of results) {
        console.log("running statement for", bench)
        await stmt.run(
            id,
            bench.lib,
            bench.project,
            bench.mean,
            bench.stddev,
            bench.median,
            bench.user,
            bench.system,
            bench.min,
            bench.max,
        )
    }
    await stmt.finalize();
    await db.close();
}

(async () => {
    let raw_cargo = await fs.readFile("Cargo.toml");
    let parsed = toml.parse(raw_cargo);
    let run_id = process.argv[2]
    let results_path = path.join(process.cwd(), "results", run_id);
    let files = await fs.readdir(results_path);
    let list = []
    for (let file of files) {
        let full_file_path = path.join(results_path, file);
        let raw_csv = await fs.readFile(full_file_path, "utf-8");
        let parsed_csv = await parse_csv(raw_csv);
        list = list.concat(parsed_csv);
    }
    let updated = transform_results(list, run_id);
    let run = {
        id: run_id,
        ress_version: parsed.dependencies.ress.tag,
        ressa_version: parsed.dependencies.ressa.tag,
    };
    await write_run(updated, run.id, run.ressa_version, run.ress_version);
})().catch(e => {
    console.error("Failed post processing", e)
    process.exit(1)
});
