
pub fn bench_ress(js: &str) {
    let toks = ress::tokenize(js).unwrap();
    std::mem::forget(toks);
}

pub fn bench_ressa(js: &str, is_module: bool) {
    let mut p = ressa::Parser::builder()
        .module(is_module)
        .js(js)
        .build()
        .unwrap();
    let ast = p.parse().unwrap();
    std::mem::forget(ast);
}
