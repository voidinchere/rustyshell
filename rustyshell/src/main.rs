use std::process::Command;

fn exec(cmd: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    // create a variable called "input" and then start a loop where we will take the user's input then save it to the variable and run the exec() function
    let mut input = String::new();
    println!("Enter a command to run or type 'exit' to quit:");
    loop {
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        }
        exec(input.trim().to_string());
        input.clear();
    }
}