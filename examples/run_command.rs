pub fn execute_task() {
    use std::process::Command;

    let output = Command::new("touch")
        .arg("hello.txt")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;

    println!("{:?}", output.status);
}

fn main() {
    execute_task();
}
