.PHONY: all test

all: test src/ue_sim_api.rs

test:
	-pkill web
	cargo build
	cargo run &
	./test.bash

src/ue_sim_api.rs: src/ue-sim-api.h
	bindgen --no-layout-tests $^ -o $@

