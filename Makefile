isort = isort pytokei tests
black = black pytokei tests

develop:
	maturin develop

test:
	pytest tests

mypy:
	mypy pytokei

format:
	$(isort)
	$(black)
	cargo fmt
