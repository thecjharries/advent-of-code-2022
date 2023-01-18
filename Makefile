ifndef DAY
$(error DAY is not set)
endif

.PHONY: new
new:
	cargo new day-$(DAY) --vcs none
