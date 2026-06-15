// Loops, mutation, and a helper function.
func factorial(n) {
    let result = 1
    let i = 2
    while i <= n {
        result = result * i
        i = i + 1
    }
    return result
}

print("Factorials:")
for n in 1..8 {
    print(n, "! =", factorial(n))
}

// FizzBuzz
print("FizzBuzz 1..15:")
for n in 1..16 {
    if n % 15 == 0 {
        print("FizzBuzz")
    } else if n % 3 == 0 {
        print("Fizz")
    } else if n % 5 == 0 {
        print("Buzz")
    } else {
        print(n)
    }
}
