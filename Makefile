new-day-%:
	@echo "Creating day $*"
	mkdir aoc-2025-inputs/day-$*/
	touch aoc-2025-inputs/day-$*/input.txt
	touch aoc-2025-inputs/day-$*/test.txt
	cargo generate template-day --name day-$* -d day=$*
