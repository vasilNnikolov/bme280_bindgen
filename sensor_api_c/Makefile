# Variables
CC = gcc
AR = ar
CFLAGS = -Wall -Wextra -pedantic
LIB_NAME = libmylibrary.a

# Source files
SRCS = bme280.c
OBJS = $(SRCS:.c=.o)

# Main program
MAIN_SRC = main.c

# Default target
all: $(LIB_NAME) 

# Compile source files into object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Create static library from object files
$(LIB_NAME): $(OBJS)
	$(AR) rcs $@ $^

# # Compile the main program with the static library
# $(TARGET): $(MAIN_SRC) $(LIB_NAME)
# 	$(CC) $(CFLAGS) -o $@ $(MAIN_SRC) -L. -lmylibrary

# Clean up build files
clean:
	rm -f $(OBJS) $(LIB_NAME) $(TARGET)

# PHONY targets
.PHONY: all clean
