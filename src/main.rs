use std::io::{self, Write};

struct TodoList {
    todos: Vec<String>,
    archived: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            archived: Vec::new(),
        }
    }

    fn display_todos(&self) {
        println!("List of the last 5 added todos :
        ");
        for (i, todo) in self.todos.iter().rev().take(5).enumerate() {
            println!("{}. {}", i + 1, todo);
        }
    }

    fn archive_todo(&mut self, num: usize) {
        if num > 0 && num <= self.todos.len() {
            let archived_todo = self.todos.remove(self.todos.len() - num);
            self.archived.push(archived_todo);
            println!("The todo has been archived.");
        } else {
            println!("Invalid todo number.");
        }
    }

    fn display_archived_todos(&self) {
        println!("Archived todos :");
        for (i, todo) in self.archived.iter().enumerate() {
            println!("{}. {}", i + 1, todo);
        }
        println!("Appuyez sur'q' pour revenir à la vue de base.");
    }

    fn add_todo(&mut self, new_todo: String) {
        self.todos.push(new_todo);
        println!("La todo a été ajoutée.");
    }
}

fn main() {
    /* todolsit command (a, q, n) */
    let mut todo_list = TodoList::new();

    loop {
        todo_list.display_todos();

        println!("Options:");
        println!("  - Add a todo (n)");
        println!("  - Displays archived todos (a)");
        println!("  - Archive a todo  (number of the todo)");
        println!("  - Quit (q)");

        let mut input = String::new();
        print!("Chose an option : ");
        io::stdout().flush().expect("Error");

        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        
        match input.trim() {
            "n" => {
                println!("Enter a new todo : ");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Error");
                todo_list.add_todo(new_todo.trim().to_string());
            }

            "a" => {
                todo_list.display_archived_todos()
            }
            
            "q" => {
                break;
            }
            
            _ => {
                if let Ok(num) = input.trim().parse::<usize>() {
                    todo_list.archive_todo(num);
                } else {
                    println!("Commande invalide. Veuillez réessayer.");
                }
            }
        }   

    
    
    
    
    }
}
