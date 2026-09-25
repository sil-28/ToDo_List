

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
    for (i,task) in list.iter().enumerate(){
        let status = if task.done {"x"} else {""};
        println!("{}. [{}] {}", i, status, task.description);
    }
}


fn main() {
    let mut list: Vec<Task> = Vec::new();




}
