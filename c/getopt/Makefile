CC := gcc
CFLAGS = -Wall -Werror -std=c11
LDFLAGS = -Wall

all: main.out

%.out:%.o
	$(CC) $< -o $@ $(LDFLAGS)

%.o:%.c
	$(CC) -c $< $(CFLAGS)
