# Compiler to use
CC = gcc

# Compiler flags:
# -Wall: Enable all warning messages
# -Wextra: Enable extar warning flags
# -g: Include debugging information
# -pedantic: Strict compliance with C and C++ standard
# -std=c11: Compile according to C11 standard
CFLAGS = -Wall -Wextra -g -pedantic -std=c11

# Target executable name
TARGET = matching

# Source files
SRCS = main.c lib.c recursive_match.c matrix.c

# Default rule to build the target
all: $(TARGET)

# Link object files to create the executable
$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRCS)

# Clean up build files
clean:
	rm -f $(OBJS) $(TARGET)