# Makefile per lcov2generic
# Obiettivi: build, test, coverage, publish

CARGO ?= cargo

.PHONY: all build test coverage publish release tag

all: build test

## 🔨 Build del progetto
build:
	$(CARGO) build --release

## 🧪 Esecuzione test
test:
	$(CARGO) test --all --verbose

## 📊 Generazione coverage LCOV + conversione in Generic XML
coverage:
	$(CARGO) llvm-cov --workspace --locked --lcov --output-path lcov.info
	lcov2generic lcov.info generic-coverage.xml

## 🚀 Pubblicazione su crates.io (usa il token configurato con cargo login)
publish:
	$(CARGO) publish

## 🏷️ Creazione tag di release e push su GitHub
# Uso: make release VERSION=0.1.0
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "❌ Devi specificare VERSION, es: make release VERSION=0.1.0"; \
		exit 1; \
	fi
	$(CARGO) set-version $(VERSION)
	git commit -am "Release v$(VERSION)"
	git tag v$(VERSION)
	git push origin main --tags

## 🔖 Shortcut per creare un tag senza bump automatico
tag:
	@if [ -z "$(VERSION)" ]; then \
		echo "❌ Devi specificare VERSION, es: make tag VERSION=0.1.0"; \
		exit 1; \
	fi
	git tag v$(VERSION)
	git push origin v$(VERSION)
