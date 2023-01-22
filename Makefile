ifndef DAY
$(error DAY is not set)
endif

ifneq (,$(wildcard day-$(DAY)))
$(error day-$(DAY) already exists)
endif

.PHONY: new
new:
	cargo new day-$(DAY) --vcs none
