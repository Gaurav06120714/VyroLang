// Maps and the match expression.

// --- Maps ---
let inventory = { "apples": 5, "bananas": 3 }
inventory["cherries"] = 12

let ks = keys(inventory)
let total = 0
for i in 0..len(ks) {
    let item = ks[i]
    print(item + ": " + str(inventory[item]))
    total = total + inventory[item]
}
print("total items:", total)
print("has apples?", has(inventory, "apples"))
print("has mangoes?", has(inventory, "mangoes"))

// --- match ---
func classify(n) {
    return match n {
        0 -> "zero"
        1 -> "one"
        _ -> "many"
    }
}

for n in 0..4 {
    print(n, "is", classify(n))
}
