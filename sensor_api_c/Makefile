# Compiler and flags
CC = gcc
CFLAGS = -I./BME280_SensorAPI -Wall -Werror

# Source directory
SRC_DIR = BME280_SensorAPI

# Build directory
BUILD_DIR = build

# Source files
SOURCES = $(wildcard $(SRC_DIR)/*.c)

# Object files placed in build directory
OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(SOURCES:.c=.o)))

# Output library name in build directory
TARGET = $(BUILD_DIR)/libbme280.a

# Default rule
all: $(BUILD_DIR) $(TARGET)

# Make sure the build directory exists
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# Static library rule
$(TARGET): $(OBJECTS)
	@echo "Creating static library $@"
	ar rcs $@ $(OBJECTS)

# Compile rule
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@echo "Compiling $< into $@"
	$(CC) $(CFLAGS) -c $< -o $@

# Clean rule
clean:
	@echo "Cleaning up..."
	rm -f $(OBJECTS) $(TARGET)

.PHONY: all clean
