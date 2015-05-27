CC := clang++
CCFLAGS := -std=c++11 -Wall -Wextra -O2
LDFLAGS :=
OBJS = main.o

default: a.out

a.out: $(OBJS)
	$(CC) $^ -o $@ $(LDFLAGS)

%.o:%.cc
	$(CC) -c $< $(CCFLAGS)

clean:
	rm -f $(OBJS) a.out
