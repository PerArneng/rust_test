BIN_DIR=target
SRC_DIR=src

all: hello_world task_test hello_server cps_error_handling

cps_error_handling: $(SRC_DIR)/cps_error_handling.rs
	mkdir -p $(BIN_DIR)
	rustc --out-dir $(BIN_DIR) $(SRC_DIR)/cps_error_handling.rs

hello_world: $(SRC_DIR)/hello_world.rs
	mkdir -p $(BIN_DIR)
	rustc --out-dir $(BIN_DIR) $(SRC_DIR)/hello_world.rs

task_test: $(SRC_DIR)/hello_world.rs
	mkdir -p $(BIN_DIR)
	rustc --out-dir $(BIN_DIR) $(SRC_DIR)/task_test.rs

hello_server: $(SRC_DIR)/hello_server.rs
	mkdir -p $(BIN_DIR)
	rustc --out-dir $(BIN_DIR) $(SRC_DIR)/hello_server.rs

clean:
	rm -rf $(BIN_DIR)