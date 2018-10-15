package main

import (
	"fmt"
)

func main() {
	nums := []int{2, 3, 5, 6, 4}
	sum := 11
	fmt.Println(twosum(nums, sum))
}

func twosum(nums []int, sum int) []int {
	index := make(map[int]int, len(nums))
	for i, v := range nums {
		if j, ok := index[sum-v]; ok {
			return []int{j, i}
		}
		index[v] = i
	}
	return nil
}
