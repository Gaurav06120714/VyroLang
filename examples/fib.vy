// Recursive Fibonacci — exercises functions, recursion, conditionals.
func fib(n) {
    if n < 2 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

for i in 0..10 {
    print("fib(" + i + ") =", fib(i))
}
