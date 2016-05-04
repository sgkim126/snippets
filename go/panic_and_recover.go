package main

import (
	"fmt"
)

type SomeError struct {
	I int
	S string
}

type AnotherError struct {
	I string
	S int
}

func main() {
	f()
	fmt.Printf("Haaaaaaaaaaaaa\n")
}

func f() {
	defer func() {
		r := recover()
		if r != nil {
			val, ok := r.(AnotherError)
			fmt.Printf("Recover %t %d %s\n", ok, val.I, val.S)
		}
	}()
	g()
}

func g() {
	s := SomeError{1, "a"}
	fmt.Printf("Panic %s %p\n", s, &s)
	panic(s)
}
