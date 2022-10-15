isort = isort pytokei tests
black = black pytokei tests

develop:
	maturin develop

test:
	pytest tests

format:
	$(isort)
	$(black)
	cargo fmt
