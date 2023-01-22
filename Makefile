YEAR=2022

ifeq (,$(DAY))
	DAY=$(shell printf "%02d" $(shell expr $(shell find . -maxdepth 1 -type d -name 'day-[0-9][0-9]' | sort -r | head -n 1 | sed -e 's/[^0-9]//g') + 1))
endif

ifndef DAY
$(error DAY is not set)
endif

ifneq (,$(wildcard day-$(DAY)))
$(error day-$(DAY) already exists)
endif

NONZERO_DAY=$(shell echo $(DAY) | sed 's/^0*//')

.PHONY: new
new:
	git checkout -b feat/day-$(DAY)
	cargo new day-$(DAY) --vcs none
	rm -rf day-$(DAY)/src/main.rs
	cp ./boilerplate.rs day-$(DAY)/src/main.rs
	curl --cookie "$$SESSION_COOKIE" https://adventofcode.com/$(YEAR)/day/$(NONZERO_DAY)/input > day-$(DAY)/input.txt
	cd day-$(DAY) && cargo run || exit 0
	git add .
	git commit -am 'Add day $(DAY) boilerplate'
