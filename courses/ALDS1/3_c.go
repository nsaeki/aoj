package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func newScanner() *bufio.Scanner {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	return scanner
}

func scanInt() int {
	sc.Scan()
	v, _ := strconv.Atoi(sc.Text())
	return v
}

func scanString() string {
	if sc.Scan() {
		return sc.Text()
	}
	panic(sc.Err())
}

var sc = newScanner()

type node struct {
	key        int
	prev, next *node
}

func delete(n *node) {
	n.prev.next, n.next.prev = n.next, n.prev
}

func main() {
	n := scanInt()
	f, l := &node{-1, nil, nil}, &node{-1, nil, nil}
	f.next = l
	l.prev = f

	for i := 0; i < n; i++ {
		op := scanString()
		switch op {
		case "insert":
			x := scanInt()
			t := &node{x, f, f.next}
			f.next, f.next.prev = t, t
		case "delete":
			x := scanInt()
			a := f.next
			for a != nil {
				if a.key == x {
					delete(a)
					break
				}
				a = a.next
			}
		case "deleteFirst":
			delete(f.next)
		case "deleteLast":
			delete(l.prev)
		}
	}

	w := bufio.NewWriter(os.Stdout)
	a := f.next
	for a != nil && a.key != -1 {
		fmt.Fprint(w, a.key)
		a = a.next
		if a != nil && a.key != -1 {
			fmt.Fprint(w, " ")
		}
	}
	fmt.Fprintln(w)
	w.Flush()
}
