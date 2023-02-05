#! /bin/bash

BASE_OUT_DIR="results/$(date -I)"
OUT_DIR=$BASE_OUT_DIR
NUM=0
while [ -d "$OUT_DIR" ]; do
    NUM=$((NUM+1))
    OUT_DIR="$BASE_OUT_DIR-$NUM"
done

mkdir $OUT_DIR

for path in bins/*; do
    name=$(basename $path)
    hyperfine -N $path --export-csv "$OUT_DIR/$name.csv"
    # sleep 5
done
node scripts/post_run.js $(basename $OUT_DIR)
