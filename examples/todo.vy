// A small Todo app in VyroLang — exercises classes, arrays, the stdlib, and loops.
// (A trimmed version of the Todo reference app from docs/09-applications.)

class Task {
    title
    priority
    done
    func init(title, priority) {
        self.title = title
        self.priority = priority
        self.done = false
    }
    func complete() {
        self.done = true
    }
    func label() {
        let mark = "[ ]"
        if self.done {
            mark = "[x]"
        }
        return mark + " (p" + self.priority + ") " + self.title
    }
}

class TodoList {
    func init() {
        self.tasks = []
    }
    func add(title, priority) {
        push(self.tasks, Task(title, priority))
    }
    func completeAt(i) {
        let t = self.tasks[i]
        t.complete()
    }
    func show() {
        for i in 0..len(self.tasks) {
            let t = self.tasks[i]
            print(t.label())
        }
    }
    func remaining() {
        let count = 0
        for i in 0..len(self.tasks) {
            let t = self.tasks[i]
            if !t.done {
                count = count + 1
            }
        }
        return count
    }
}

let list = TodoList()
list.add("Write VyroLang compiler", 3)
list.add("Build the VM", 3)
list.add("Wire into VyroCoding", 2)
list.add("Sleep", 1)

list.completeAt(0)
list.completeAt(1)

print("=== " + upper("todo") + " ===")
list.show()
print("Remaining:", list.remaining())
