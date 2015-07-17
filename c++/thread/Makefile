CC := g++
CCFLAGS := -std=c++11 -Wall
LDFLAGS := -Wall -pthread
OBJS = main.o

default: a.out

a.out: $(OBJS)
	$(CC) $^ -o $@ $(LDFLAGS)

%.o:%.cc
	$(CC) -c $< $(CCFLAGS)

clean:
	rm -f $(OBJS) a.out
