.SUFFIXES:
CC := gcc
NAME :=
SRCS := $(patsubst %,%.c,$(NAME))
OBJS := $(patsubst %,%.o,$(NAME))
TARG :=
TEST :=
LIBS =
CFLAGS =
LDFLAGS =
DEP := .depend
.PHONY: all test clean install error dep
.DEFAULT_GOAL:=test

test: $(TEST)
	make all CFLAGS='-g'
all: $(TARG)
install:all
	echo "Not Implemented"

$(TARG): $(OBJS)
	$(CC) $(LDFLAGS) $? -o $@ -Wall

%.out:%.o
	$(CC) $< -o $@ $(LDFLAGS) -Wall
%.o:%.c
	$(CC) -c $< $(CFLAGS)

clean:
	-rm -f $(TARG)
	-rm -f $(OBJS)
cleandep:
	-rm -f $(DEP)

dep:
	$(CC) -M $(INC) $(SRCS) > $(DEP)

.DEFAULT:
	echo "Error: " $< "is not implemented."

ifeq ($(DEP),$(wildcard $(DEP)))
include .depend
endif
