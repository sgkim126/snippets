package main

import (
	"bytes"
	"compress/gzip"
	"fmt"
	"io/ioutil"
)

func main() {
	var original = []byte("Some string")
	var encoded bytes.Buffer
	writer := gzip.NewWriter(&encoded)
	if n, err := writer.Write(original); err != nil {
		fmt.Errorf("%#v", err)
	} else {
		fmt.Printf("%d write\n", n)
	}
	writer.Close()

	fmt.Printf("original:%s:\n", original)
	fmt.Printf("encoded:%s:\n", encoded)

	reader, err := gzip.NewReader(&encoded)
	if err != nil {
		fmt.Errorf("%#v", err)
	}

	decoded, err2 := ioutil.ReadAll(reader)
	if err2 != nil {
		fmt.Errorf("%#v", err)
	}
	if err := reader.Close(); err != nil {
		fmt.Errorf("%#v", err)
	}

	fmt.Printf("decoded:%s:\n", decoded)
}
