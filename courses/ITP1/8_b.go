package main

import "fmt"

func main() {
	for {
		var x string
		fmt.Scan(&x)
		if x == "0" {
			break
		}

		ans := 0
		for i := 0; i < len(x); i++ {
			ans += int(x[i]-'0') % 10
		}
		fmt.Println(ans)
	}
}
