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
	cd day-$(DAY) && cargo run || exit 0
	git add .
	git commit -am 'Add day $(DAY) boilerplate'
	curl --cookie "$$SESSION_COOKIE" https://adventofcode.com/2020/day/$(NONZERO_DAY)/input > day-$(DAY)/input.txt
