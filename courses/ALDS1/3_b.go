package main

import (
	"bufio"
	"container/list"
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

type proc struct {
	name      string
	time, fin int
}

func main() {
	n, q := scanInt(), scanInt()
	queue := list.New()
	f := make([]*proc, 0, n)
	for i := 0; i < n; i++ {
		p := &proc{scanString(), scanInt(), 0}
		queue.PushBack(p)
	}

	total := 0
	for queue.Len() > 0 {
		e := queue.Front()
		queue.Remove(e)
		p := e.Value.(*proc)
		if p.time > q {
			total += q
		} else {
			total += p.time
		}
		p.time -= q
		if p.time <= 0 {
			p.fin = total
			f = append(f, p)
		} else {
			queue.PushBack(p)
		}
	}
	for i := 0; i < n; i++ {
		fmt.Println(f[i].name, f[i].fin)
	}
}
