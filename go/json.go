package main

import (
	"encoding/json"
	"fmt"
)

type A struct {
	A int
	B string
	C bool
}

func main() {
	a := A{10324, "xyz", true}
	j, err := json.Marshal(a)
	if err != nil {
		fmt.Errorf(":%v:\n", err)
	}
	fmt.Printf(":%s:\n", j)

	b := new(A)
	json.Unmarshal(j, b)
	fmt.Printf(":%s:\n", b)
}
