CXX ?= g++
CXXFLAGS ?= -O3 -std=c++20 -march=native -Wall -Wextra
LDFLAGS ?= 

BIN_DIR := bin
SRC_DIR := src

all: $(BIN_DIR)/parse

$(BIN_DIR)/parse: $(SRC_DIR)/parse.cpp
	@mkdir -p $(BIN_DIR)
	$(CXX) $(CXXFLAGS) $< -o $@ $(LDFLAGS)

clean:
	rm -rf $(BIN_DIR)
