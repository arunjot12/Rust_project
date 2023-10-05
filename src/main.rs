use std::io;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();  
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn convert(string: String   ) -> u32 {
    string.trim().parse().expect("Please type a number!")
}

fn in_hours() {
    let mut date = String::new();
    let mut employee_number = String::new();
    let mut in_timings = String::new();
    
    println!("Enter the date");
    io::stdin().read_line(&mut date).unwrap();
   
    println!("Enter the details of the employee");
    println!("Enter the employee number");
    io::stdin().read_line(&mut employee_number).expect("Failed to read line");
    convert(employee_number);
    println!("Enter the In timing of the employee ");
    io::stdin().read_line(&mut in_timings).expect("Failed to read line");


}

fn out_hours() {
    println!("Enter the details of the employeee");

    println!("Enter the employee number");
    let mut employee_number = String::new();
    io::stdin().read_line(&mut employee_number).expect("Failed to read line");
    convert(employee_number);
    println!("Enter the In timing of the employee leaving ");
    let mut out_timings = String::new();
    io::stdin().read_line(&mut out_timings).expect("Failed to read line");
}

fn add() {
    println!("\nDo you want to add the:\n'InHours'\n'OutHours'\n'Exit'");
    loop {
        let mut add = String::new();
        io::stdin().read_line(&mut add).unwrap();
        let command: String = add.to_lowercase().trim().parse().unwrap();
        match command.as_ref() {
            // "show" => show_groceries(),
            "inhours" => in_hours(),
            "OutHours" => out_hours(),
            // "delete" => delete_grocery(),
            "exit" => {
                break;
            }
            _ => println!("Not a correct key word"),
        }
    }
}

fn main() {
    println!("Welcome to the attendance system here");
    println!("\nDo you want to:\n'Show'\n'Add'\n'Exit'");
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: String = command.to_lowercase().trim().parse().unwrap();
        match command.as_ref() {
            // "show" => show_groceries(),
            "add" => add(),
            // // "change" => update_grocery(),
            // "delete" => delete_grocery(),
            "exit" => {
                break;
            }
            _ => println!("Not a correct key word"),
        }
    }
}
