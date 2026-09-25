use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;

struct Task {
    description: String,
    done: bool
}

fn add_task(list: &mut Vec<Task>, description: String, file_name: &String) {
    list.push(Task { description, done: false });
    save_to_file(list, file_name);
}

fn complete_task(list: &mut Vec<Task>, index: usize,  file_name: &String){
    if let Some(task) = list.get_mut(index){
        task.done = true;
        save_to_file(list, file_name);
    }
}

fn remove_task(list: &mut Vec<Task>, index: usize,  file_name: &String){
    if index<list.len(){
        list.remove(index);
        save_to_file(list, file_name);
    }
}

fn print_list(list: &Vec<Task>){
    println!("");
    for (i,task) in list.iter().enumerate(){
        let status = if task.done {"x"} else {""};
        println!("{}. [{}] {}", i, status, task.description);
    }
}

fn save_to_file(list: &Vec<Task>, file_name: &String){
    let mut file = File::create(file_name).unwrap();
    for task in list.iter() {
        let status = if task.done {"x"} else {""};
        writeln!(file, "[{}] {}", status, task.description).unwrap();
    }
}

fn load_from_file(file_name: String)->(Vec<Task>,bool){
    
    let mut list: Vec<Task> = Vec::new();
    let content = fs::read_to_string(file_name);
    let mut check = true;

    match content {
        Ok(text) => {
            for mut line in text.lines(){
                let mut done = false;
                
                if line.starts_with("[x] ") {
                    done = true;
                    line = line.strip_prefix("[x] ").unwrap();
                } else if line.starts_with("[] "){
                    line = line.strip_prefix("[] ").unwrap();
                } else {
                    println!("File format not compatible. Saving in new file tasks.txt");
                    check = false;
                    break;
                }
                
                list.push(Task { description: line.to_string(), done });
            }
            (list,check)
        },

        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!("File not founded. Saving list in new file");
            (Vec::new(), true)
        },
       
        Err(err) => {
            println!("Warning: could not read file: {}", err);
            (Vec::new(), false)
            
        }
    }
    

}


fn main() {
    let mut list: Vec<Task> = Vec::new();

    let mut load_answer = String::new();
    let mut load_file_name = String::new();
    let mut loaded_from_file = false;
    let mut check= false;

    loop{
        println!("Do you want to load a file? [Y/N]");
        load_answer.clear();
        io::stdin().read_line(&mut load_answer).expect("Failed to read line");
        let load = load_answer.trim();
        match load {
            "Y" | "y" => {println!("Write the name of the file txt (without the extencion)");
                    io::stdin().read_line(&mut load_file_name).expect("Failed to read line");
                    (list,check) = load_from_file(load_file_name.trim().to_owned() + ".txt");
                    loaded_from_file = true;
                    break;
                },
            "N" | "n" => break,
            _ => println!("Please enter a valid answer")
        };
    }

    let save_file_name;
    if loaded_from_file & check {
        save_file_name = load_file_name.trim().to_owned() + ".txt";
    } else {
        save_file_name = "tasks.txt".to_string();
    }

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
                add_task(&mut list, description.trim().to_string(), &save_file_name);
                },
            "2" => {
                println!("Enter task index to complete");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");

                let index = index.trim().parse::<usize>();
                match index{
                    Ok(i) => {if i<list.len(){complete_task(&mut list, i, &save_file_name)}
                        else {println!("Task not found")}
                        },
                    Err(_) => println!("Please enter a valid number")
                };
                
                },
            "3" => {
                println!("Enter task index to remove");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index = index.trim().parse::<usize>();
                match index{
                    Ok(i) => {if i<list.len(){remove_task(&mut list, i, &save_file_name)}
                        else {println!("Task not found")}
                        },
                    Err(_) => println!("Please enter a valid number")
                };
                
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
