package main

import (
	"fmt"
)

var match = map[rune]rune{
	')': '(',
	']': '[',
	'}': '{',
}

type stack []rune

func main() {
	fmt.Println(isValid("{sdfa[]()}"))
}

func isValid(str string) bool {
	if len(str) < 2 {
		return false
	}

	s := new(stack)
	for _, v := range str {
		switch v {
		case '(', '[', '{':
			s.push(v)
		case ')', ']', '}':
			if res, ok := s.pop(v); !ok || res != match[v] {
				return false
			}
		}
	}

	if len(*s) == 0 {
		return true
	}
	return false

}

func (s *stack) pop(a rune) (rune, bool) {
	if len(*s) > 0 {
		// pop的元素
		res := (*s)[len(*s)-1]
		*s = (*s)[:len(*s)-1]
		return res, true
	}

	return 0, false

}

func (s *stack) push(b rune) {
	*s = append(*s, b)
}
