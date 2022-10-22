isort = isort pytokei tests
black = black pytokei tests

develop:
	maturin develop

test:
	pytest tests

mypy:
	mypy pytokei

format-python:
	$(isort)
	$(black)

format-rust:
	cargo fmt

format: format-python, format-rust

install-dev:
	pip install -r requirements/all.txt

docs:
	mkdocs build
