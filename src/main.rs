use dialoguer::Confirm;
use std::collections::HashMap;
fn main() {
    println!("\r\x1b[2J\r\x1b[H"); //\r is for going left, [2j clear screen, [H move cursor to home

    println!(
        "Use the following commands:

[Add to department] > \"ADD <person> TO <department>\"

[List all in department] > \"LIST FROM <department>\"

[List all in company] > \"LIST FROM ALL\"
    "
    );

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        input = input.trim().to_string();

        let args: Vec<&str> = input.split_whitespace().collect();

        let input_lower = input.clone().to_lowercase(); //for case insensitive matching
        let mut args_match: Vec<&str> = input_lower.split_whitespace().collect(); //Split into a vec

        //Match the input command
        match &mut args_match[..] {
            ["add", _, "to", _] => add_emp(&mut company, args[1], args[3]), //Use the args from the original input
            ["list", "from", "all"] => list(&mut company, "all"),
            ["list", "from", _] => list(&mut company, args[2]),
            _ => println!("Invalid command: \"{input}\""),
        }
    }
}

fn add_emp(company: &mut HashMap<String, Vec<String>>, emp: &str, dep: &str) {
    if let Ok(false) = Confirm::new()
        .with_prompt(format!("Are you sure you want to add {emp} to {dep}?"))
        .interact()
    {
        println!("Cancelling..");
        return;
    }

    println!("Adding {emp} to {dep}");

    company
        .entry(String::from(dep))
        .or_insert(Vec::new()) //Create new vector for department if not exists
        .push(String::from(emp)); //Else append to it
}

fn list(company: &mut HashMap<String, Vec<String>>, from: &str) {
    if from == "all" {
        let mut sorted_departments: Vec<String> = company.keys().cloned().collect(); //Clone each key
        sorted_departments.sort_by_key(|d| d.to_lowercase()); //Sort keys alphabetically

        for department in &sorted_departments {
            println!("\n[{department}]");
            list_employees(company, department);
        }

        return;
    }

    println!("[{from}]");
    list_employees(company, from);
}

fn list_employees(company: &mut HashMap<String, Vec<String>>, department: &str) {
    let Some(employees) = company.get_mut(department) else { return };

    employees.sort_by_key(|e| e.to_lowercase());

    for employee in employees {
        println!("{employee}");
    }
}
