package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	ch := make(chan string)
	var wg sync.WaitGroup

	for i := 0; i < 5; i++ {
		i := i
		wg.Add(1)
		go func() {
			defer wg.Done()

			for {
				it, ok := <-ch
				if !ok {
					break
				}
				time.Sleep(100 * time.Millisecond)
				fmt.Printf("worker %d received %s\n", i, it)
			}
		}()
	}

	for i := 0; i < 100; i++ {
		ch <- fmt.Sprintf("msg %d", i)
	}

	close(ch)

	wg.Wait()
}
