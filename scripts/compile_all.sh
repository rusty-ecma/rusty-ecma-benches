#! /bin/bash

BIN_PATH=$(cargo build --release --bin ress_angular --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_angular --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_angular_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_angular_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_dexie --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_dexie --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_dexie_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_dexie_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_everything --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_everything --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_everything2015 --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_everything2015 --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_everything2015_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_everything2015_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_jquery --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_jquery --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_jquery_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_jquery_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_moment --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_moment --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_moment_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_moment_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_moment_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_moment_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_react --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_react --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_react_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_react_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_react_dom --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_react_dom --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_react_dom_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_react_dom_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_vue_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_vue_min --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ress_vue_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins

BIN_PATH=$(cargo build --release --bin ressa_vue_mod --message-format json | tail -n2 | head -n1 | jq -r '.executable')
cp $BIN_PATH ./bins
