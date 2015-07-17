#!/usr/bin/env python
import markdown


def convert(filename):
    content = ''.join(open(filename).readlines())
    return markdown.markdown(content, extensions=['attr_list'])


if __name__ == '__main__':
    print(convert('sample.md'))
