package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := 12321
	fmt.Println("is :", palindromeNumber(n))
}

func palindromeNumber(n int) bool {
	if n < 0 {
		return false
	}

	ns := strconv.Itoa(n)

	for i, j := 0, len(ns)-1; i < j; i, j = i+1, j-1 {
		if ns[i] != ns[j] {
			return false
		}
	}
	return true

}
