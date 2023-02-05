use handlebars::Handlebars;
use std::{path::PathBuf, io::Write, os::unix::prelude::PermissionsExt};

const RESS_TEMPLATE: &str = include_str!("templates/ress_bin.rs.handlebars");
const RESSA_TEMPLATE: &str = include_str!("templates/ressa_bin.rs.handlebars");

const LIBS: &[MajorLib] = &[
    MajorLib::Angular,
    MajorLib::Dexie,
    MajorLib::Everything,
    MajorLib::Everything2015,
    MajorLib::Jquery,
    MajorLib::Moment,
    MajorLib::React,
    MajorLib::ReactDom,
    MajorLib::Vue,
];

fn main() {
    let proj_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scripts_dir = proj_dir.join("scripts");
    let src_dir = proj_dir.join("src");
    let bin_dir = src_dir.join("bin");
    std::fs::remove_dir_all(&bin_dir).ok();
    std::fs::create_dir_all(&bin_dir).expect("Failed to create src/bin");
    
    let reg = Handlebars::new();
    let mut bin_names = Vec::new();
    for &lib in LIBS {
        if let Some(js_path) = lib.path() {
            let ress_bin = reg.render_template(RESS_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ress-{}: {e}", lib.name());
            });
            let bin_name = format!("ress_{}", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            std::fs::write(&bin_path, ress_bin).unwrap();

            let bin_name = format!("ressa_{}", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            let ressa_bin = reg.render_template(RESSA_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
                "is_module": false,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ressa-{}: {e}", lib.name());
            });
            std::fs::write(&bin_path, ressa_bin).unwrap();
        }
        if let Some(js_path) = lib.path_min() {
            let ress_bin = reg.render_template(RESS_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ress-{}: {e}", lib.name());
            });
            let bin_name = format!("ress_{}_min", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            std::fs::write(&bin_path, ress_bin).unwrap();

            let bin_name = format!("ressa_{}_min", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            let ressa_bin = reg.render_template(RESSA_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
                "is_module": false,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ressa-{}: {e}", lib.name());
            });
            std::fs::write(&bin_path, ressa_bin).unwrap();
        }
        if let Some(js_path) = lib.path_mod() {
            let ress_bin = reg.render_template(RESS_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ress-{}: {e}", lib.name());
            });
            let bin_name = format!("ress_{}_mod", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            std::fs::write(&bin_path, ress_bin).unwrap();
            
            
            let bin_name = format!("ressa_{}_mod", lib.name());
            bin_names.push(bin_name.clone());
            let bin_path = bin_dir.join(&format!("{bin_name}.rs"));
            let ressa_bin = reg.render_template(RESSA_TEMPLATE, &serde_json::json!({
                "js_path": js_path,
                "is_module": true,
            })).unwrap_or_else(|e| {
                panic!("Failed to render ressa-{}: {e}", lib.name());
            });
            std::fs::write(&bin_path, ressa_bin).unwrap();
        }
    }
    let script_path = scripts_dir.join("compile_all.sh");
    std::fs::remove_file(&script_path).ok();
    let mut file = std::fs::File::create(&script_path).unwrap();
    file.write_all(b"#! /bin/bash\n").unwrap();
    for bin in bin_names {
        file.write_all(format!("\nBIN_PATH=$(cargo build --release --bin {bin} --message-format json | tail -n2 | head -n1 | jq -r '.executable')").as_bytes()).unwrap();
        file.write_all(format!("\ncp $BIN_PATH ./bins\n").as_bytes()).unwrap();
    }
    std::fs::set_permissions(&script_path, PermissionsExt::from_mode(0x16D)).unwrap();
}


#[derive(Debug, Clone, Copy)]
enum MajorLib {
    Angular,
    Dexie,
    Everything,
    Everything2015,
    Jquery,
    Moment,
    React,
    ReactDom,
    Vue,
}

impl MajorLib {
    const fn name(self) -> &'static str {
        match self {
            MajorLib::Angular => "angular",
            MajorLib::Dexie => "dexie",
            MajorLib::Everything => "everything",
            MajorLib::Everything2015 => "everything2015",
            MajorLib::Jquery => "jquery",
            MajorLib::Moment => "moment",
            MajorLib::React => "react",
            MajorLib::ReactDom => "react_dom",
            MajorLib::Vue => "vue",
        }
    }
    const fn path(self) -> Option<&'static str> {
        match self {
            MajorLib::Angular => Some("node_modules/angular/angular.js"),
            MajorLib::Dexie => Some("node_modules/dexie/dist/dexie.js"),
            MajorLib::Everything => Some("node_modules/everything.js/es5.js"),
            MajorLib::Everything2015 => Some("node_modules/everything.js/es2015-script.js"),
            MajorLib::Jquery => Some("node_modules/jquery/dist/jquery.js"),
            MajorLib::Moment => Some("node_modules/moment/min/moment-with-locales.js"),
            MajorLib::React => Some("node_modules/react/umd/react.development.js"),
            MajorLib::ReactDom => Some("node_modules/react-dom/umd/react-dom.development.js"),
            // TODO: this errors on lexical redecl for the name `get` once as a `const` and once
            // as a function expressions with an identifier
            // MajorLib::Vue => Some("node_modules/vue/dist/vue.global.js"),
            MajorLib::Vue => None,
        }
    }

    const fn path_min(self) -> Option<&'static str> {
        match self {
            MajorLib::Angular => Some("node_modules/angular/angular.min.js"),
            MajorLib::Dexie => Some("node_modules/dexie/dist/dexie.min.js"),
            MajorLib::Everything => None,
            MajorLib::Everything2015 => None,
            MajorLib::Jquery => Some("node_modules/jquery/dist/jquery.min.js"),
            MajorLib::Moment => Some("node_modules/moment/min/moment.min.js"),
            MajorLib::React => Some("node_modules/react/umd/react.production.min.js"),
            MajorLib::ReactDom => Some("node_modules/react-dom/umd/react-dom.production.min.js"),
            MajorLib::Vue => Some("node_modules/vue/dist/vue.global.prod.js"),
        }
    }

    const fn path_mod(self) -> Option<&'static str> {
        match self {
            MajorLib::Angular => None,
            // TODO: this has a duplicate export error since the `as` alias for `Dexie$1` should
            // be enough to not create a duplicate export
            // MajorLib::Dexie => Some("node_modules/dexie/dist/dexie.mjs"),
            MajorLib::Dexie => None,
            MajorLib::Everything => None,
            MajorLib::Everything2015 => Some("node_modules/everything.js/es2015-module.js"),
            MajorLib::Jquery => None,
            MajorLib::Moment => Some("node_modules/moment/dist/moment.js"),
            MajorLib::React => None,
            MajorLib::ReactDom => None,
            MajorLib::Vue => Some("node_modules/vue/dist/vue.esm-browser.js"),
        }
    }

}
