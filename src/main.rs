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
        println!("Liste des 5 dernières todos ajoutées:");
        for (i, todo) in self.todos.iter().rev().take(5).enumerate() {
            println!("{}. {}", i + 1, todo);
        }
    }

    fn archive_todo(&mut self, num: usize) {
        if num > 0 && num <= self.todos.len() {
            let archived_todo = self.todos.remove(self.todos.len() - num);
            self.archived.push(archived_todo);
            println!("La todo a été archivée.");
        } else {
            println!("Numéro de todo invalide.");
        } 
    }

    fn display_archived_todos(&self) {
        println!("Todos archivées :");
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



}
