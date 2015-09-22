package main

import "fmt"
import unsafe "unsafe"

type EmptyStruct struct {
}

func main() {
	var empty_struct EmptyStruct
	fmt.Println("{} {}", unsafe.Sizeof(empty_struct))
}
