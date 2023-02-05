const fs = require("fs").promises;
const path = require("path");
const Octokit = require("octokit").Octokit;
const toml = require("toml");

const find_max_tag = async (octokit, owner, repo) => {
    let tags = await octokit.request('GET /repos/{owner}/{repo}/tags', {
        owner,
        repo
    });
    if (tags.status !== 200) {
        throw new Error(`Bad status ${tags.status} ${owner} ${repo}`)
    }
    return tags.data.reduce((acc, val) => {
        let parts = val.name.replace(/^v/, "").split(".").map(p => +p);
        if (parts[0] > acc[0]) {
            return parts;
        }
        if (acc[0] > parts[0]) {
            return acc;
        }
        if (parts[1] > acc[1]) {
            return parts;
        }
        if (acc[1] > parts[1]) {
            return acc;
        }
        if (parts[2] > acc[2]) {
            return parts
        }
        return acc;
    }, [0, 0, 0])
}

const update_toml = (text, proj, tag) => {
    let re = new RegExp(`${proj} = { git = "https://github.com/rusty-ecma/${proj}", tag = ".+"}`)
    return text.replace(re, `${proj} = { git = "https://github.com/rusty-ecma/${proj}", tag = "${tag}"}`);
}

(async () => {
    const token = await fs.readFile(
        path.join(process.env["HOME"], ".github-pat"), "utf-8"
    );
    const octokit = new Octokit({
        auth: token.trim(),
    });
    
    let max_tag_ress = await find_max_tag(octokit, "rusty-ecma", "ress");
    let ress_tag = `v${max_tag_ress[0]}.${max_tag_ress[1]}.${max_tag_ress[2]}`;
    let max_tag_ressa = await find_max_tag(octokit, "rusty-ecma", "ressa");
    let ressa_tag = `v${max_tag_ressa[0]}.${max_tag_ressa[1]}.${max_tag_ressa[2]}`;
    let cargo_toml_text = await fs.readFile("Cargo.toml", "utf-8");
    let cargo_toml = toml.parse(cargo_toml_text);
    let updated_toml = cargo_toml_text;
    if (cargo_toml.dependencies.ress.tag != ress_tag) {
        updated_toml = update_toml(updated_toml, "ress", ress_tag);
    }
    if (cargo_toml.dependencies.ressa.tag != ressa_tag) {
        updated_toml = update_toml(updated_toml, "ressa", ressa_tag);
    }
    if (cargo_toml_text != updated_toml) {
        await fs.writeFile("Cargo.toml", updated_toml);
        return "updated cargo.toml"
    }
    return "no update needed"
})().then(console.log).catch(e => {
    console.error("error", e);
    process.exit(1);
})
