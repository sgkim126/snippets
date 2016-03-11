wc: lex.yy.c
	gcc $^ -lfl -o $@
lex.yy.c: wc.lex
	flex $^

clean:
	rm -f wc lex.yy.c
