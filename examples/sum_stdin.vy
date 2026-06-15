// Reads two integers from stdin and prints their sum.
// This is the I/O style VyroCoding uses: input() reads a line, print() emits output.
//
//   echo "5
//   7" | vyro run examples/sum_stdin.vy        ->  12

let a = int(input())
let b = int(input())
print(a + b)
