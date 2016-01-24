package main

import (
	"fmt"
)

func main() {
	a := `a
	b
	c`
	b := "a\n\tb\n\tc"
	fmt.Println(a == b)
	// Output: true
}
