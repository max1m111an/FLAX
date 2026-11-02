# ============================================================
# FLAX - Makefile wrapper around the scripts/ helpers
# ============================================================
#
# Windows uses the .bat scripts, Unix-like systems use the .sh scripts.
#
#   make                    show this help
#   make run                start the app (stable toolchain)
#   make run ARGS=--upd     start and update dependencies first
#   make dev                start the app (nightly + Cranelift)
#   make test               run the Rust unit tests
#   make test ARGS=mealy    run only tests matching a name filter
#
# All scripts are invoked from the repository root, which is what they
# expect (they look for package.json / src-tauri in the current directory).

ifeq ($(OS),Windows_NT)
    RUN     := scripts\run.bat
    RUN_DEV := scripts\run.dev.bat
    TESTS   := scripts\run.tests.bat
else
    RUN     := bash scripts/run.sh
    RUN_DEV := bash scripts/run.dev.sh
    TESTS   := bash scripts/run.tests.sh
endif

.DEFAULT_GOAL := help

.PHONY: help run run-upd dev dev-upd dev-clean dev-upd-clean test

help:
	@echo FLAX - available make targets
	@echo   make run               start app on stable toolchain
	@echo   make run-upd           update dependencies, then start
	@echo   make dev               start app on nightly with Cranelift
	@echo   make dev-upd           update nightly and Cranelift, then start
	@echo   make dev-clean         clean src-tauri before starting
	@echo   make dev-upd-clean     full update and clean, then start
	@echo   make test              run Rust unit tests
	@echo   make test ARGS=mealy   run only tests matching a filter
	@echo Scripts live in scripts/ - .bat on Windows, .sh elsewhere.

run:
	$(RUN) $(ARGS)

run-upd:
	$(RUN) --upd

dev:
	$(RUN_DEV) $(ARGS)

dev-upd:
	$(RUN_DEV) --upd

dev-clean:
	$(RUN_DEV) --clean

dev-upd-clean:
	$(RUN_DEV) --upd --clean

test:
	$(TESTS) $(ARGS)
