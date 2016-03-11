%{
int lines = 0;
int chars = 0;
int words = 0;
int in_word = 0;
%}

%%
\n {
    lines += 1;
    chars += 1;
    words += in_word;
    in_word = 0;
}
[\ \t] {
    chars += 1;
    words += in_word;
    in_word = 0;
}
. {
    chars += 1;
    in_word = 1;
}

%%
int main() {
  yylex();
  printf("\t%d\t%d\t%d\n", lines, words, chars);
}
