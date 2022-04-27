clean: ## Delete build artifacts
	@cargo clean

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

create-database:
	@sqlx db create

migrate:
	@sqlx migrate run

rollback:
	@sqlx migrate revert