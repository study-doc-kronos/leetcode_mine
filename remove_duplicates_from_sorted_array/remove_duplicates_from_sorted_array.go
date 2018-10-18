package main

import (
	"fmt"
)

func main() {
	arr := []int{1, 1, 2, 3, 3, 20, 32}
	fmt.Print(remove(arr))
}

func remove(arr []int) int {
	if len(arr) < 2 {
		return len(arr)
	}

	left, right, size := 0, 1, len(arr)
	for ; right < size; right++ {
		if arr[left] == arr[right] {
			continue
		}
		left++
		// 让右边值与左边相等
		// 持续比较即可得到不同的数字
		arr[left], arr[right] = arr[right], arr[left]
	}
	return left + 1
}
