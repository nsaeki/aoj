package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b, c float64
	fmt.Scan(&a, &b, &c)
	c *= math.Pi / 180
	h := b * math.Sin(c)
	fmt.Println(a * h / 2.0)
	fmt.Println(a + b + math.Sqrt(h*h+(a-b*math.Cos(c))*(a-b*math.Cos(c))))
	fmt.Println(h)
}
