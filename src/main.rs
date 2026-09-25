use std::io;
struct Task {
    description: String,
    done: bool
}

fn add_task(list: &mut Vec<Task>, description: String) {
    list.push(Task { description, done: false });
}

fn complete_task(list: &mut Vec<Task>, index: usize){
    if let Some(task) = list.get_mut(index){
        task.done = true;
    }
}

fn remove_task(list: &mut Vec<Task>, index: usize){
    if index<list.len(){
        list.remove(index);
    }
}

fn print_list(list: &Vec<Task>){
    println!("");
    for (i,task) in list.iter().enumerate(){
        let status = if task.done {"x"} else {""};
        println!("{}. [{}] {}", i, status, task.description);
    }
}


fn main() {
    let mut list: Vec<Task> = Vec::new();

    loop{
        println!("\n --- TO DO LIST ---");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. Remove task");
        println!("4. Show list");
        println!("5. Exit");
        println!("Choose an option:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();

        match input {
            "1" => {
                println!("Enter description of new task");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                add_task(&mut list, description.trim().to_string());
                },
            "2" => {
                println!("Enter task index to complete");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = index.trim().parse().unwrap();
                complete_task(&mut list, index);
                },
            "3" => {
                println!("Enter task index to remove");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = index.trim().parse().unwrap();
                remove_task(&mut list, index);
                },
            "4" => {
                print_list(&list);
                },
            "5" => {
                println!("Goodbye!");
                break;
                },
            _ => {
                println!("Invalid option. Choose again!");
            }
        };





    }


}
