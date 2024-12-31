struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

impl TodoList {
    fn new() -> Self {
        let new_todo_list = TodoList {
            todos: vec![],
            dones: vec![],
        };

        return new_todo_list;
    }

    fn display(&self) {
        for i in 0..self.todos.len() {
            println!("{i} [ ] - {}", self.todos[i]);
        }

        for i in 0..self.dones.len() {
            println!("  [x] - {}", self.dones[i]);
        }

        println!();
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }

    fn done(&mut self, index: usize) {
        if index >= self.todos.len() {
            println!("index given does not exist.\n");
            return;
        }

        let task = self.todos[index].clone();

        self.todos.remove(index);
        self.dones.push(task);
    }

    fn purge(&mut self) {
        if self.dones.is_empty() {
            return;
        }

        for i in (0..self.dones.len()).rev() {
            self.dones.remove(i);
        }
    }
}

impl Command {
    fn prompt() -> Self {
        let line = ftkit::read_line();
        if line.is_empty() || line.trim() == "quit" {
            return Command::Quit;
        } else if line.trim() == "purge" {
            return Command::Purge;
        }

        let task = line.strip_prefix("todo ");
        if task.is_some() {
            return Self::Todo(task.unwrap().trim().to_string());
        }

        let value = line.strip_prefix("done ");
        if value.is_some() {
            let index = value.unwrap().trim().parse::<usize>();
            if index.is_err() {
                println!("wrong argument given (number expected).\n");
                return Self::prompt();
            }

            return Self::Done(index.unwrap());
        }

        println!("command does not exist.\n");
        return Self::prompt();
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        todo_list.display();

        match Command::prompt() {
            Command::Todo(todo) => todo_list.add(todo),
            Command::Done(index) => todo_list.done(index),
            Command::Purge => todo_list.purge(),
            Command::Quit => break,
        };
    }
}
