package main

import (
	"fmt"
	"math"
)

func main() {
	for {
		var n int
		fmt.Scan(&n)
		if n == 0 {
			break
		}
		s := make([]float64, n)
		ave := 0.0
		for i := 0; i < n; i++ {
			fmt.Scan(&s[i])
			ave += float64(s[i])
		}
		ave /= float64(n)
		ans := 0.0
		for i := 0; i < n; i++ {
			ans += (s[i] - ave) * (s[i] - ave)
		}
		ans /= float64(n)
		fmt.Println(math.Sqrt(ans))
	}
}
