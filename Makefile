CC = cargo

clippy-Wall:
	$(CC) clippy -- -D warnings
