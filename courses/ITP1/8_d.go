package main

import "fmt"

func main() {
	var p, s string
	fmt.Scan(&s, &p)
	s += s
	dp := make([][]int, len(s)+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, len(p)+1)
		dp[i][0] = 0
	}

	l := 0
	for i := 1; i <= len(s); i++ {
		for j := 1; j <= len(p); j++ {
			if s[i-1] == p[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
				if l < dp[i][j] {
					l = dp[i][j]
				}
			} else {
				dp[i][j] = 0
			}
		}
	}
	if l == len(p) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
