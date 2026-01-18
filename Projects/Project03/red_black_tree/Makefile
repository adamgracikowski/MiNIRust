CC = gcc
CFLAGS = -Wall -Wextra -std=c99

CARGO = cargo
TARGET_DIR = target/release
LIB_NAME = red_black_tree

all: run

lib:
	$(CARGO) build --release

c_exec: lib
	$(CC) $(CFLAGS) main.c -L$(TARGET_DIR) -l$(LIB_NAME) -o c_demo

run: c_exec
	@echo "\n--- Running C Demo ---\n"
	LD_LIBRARY_PATH=$(TARGET_DIR) ./c_demo

clean:
	$(CARGO) clean
	rm -f c_demo

.PHONY: all lib c_exec run clean