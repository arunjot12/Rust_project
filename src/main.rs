use std::io;

fn add_hours() {
    println!("Enter the details of the employeee");
    println!("Enter the employee number");

    let mut employee_number = String::new();
    io::stdin().read_line(&mut employee_number).expect("Failed to read line");
    convert(employee_number);

    println!("Enter the In timing of the employee ");
    let mut in_timings = String::new();

    io::stdin().read_line(&mut in_timings).expect("Failed to read line");
}

fn convert(string: String) -> u32 {
    string.trim().parse().expect("Please type a number!")
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
            "add" => add_hours(),
            // // "change" => update_grocery(),
            // "delete" => delete_grocery(),
            // "exit" => break,
            _ => println!("Not a correct key word"),
        }
        println!("Thank you for using the service\nBye Byee");
    }
     
}
