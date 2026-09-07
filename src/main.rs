const TAG: &str = "fs-watch-bd3cee";
#[derive(Debug)]
enum Command { Ping, Echo(String), Count(usize), Quit }
fn execute(cmd: &Command) -> String {
    match cmd {
        Command::Ping => "PONG".to_string(),
        Command::Echo(msg) => msg.clone(),
        Command::Count(n) => format!("{}", (0..*n).sum::<usize>()),
        Command::Quit => "BYE".to_string(),
    }
}
fn main() {
    let cmds = vec![Command::Ping, Command::Echo("hello".into()), Command::Count(10), Command::Quit];
    for cmd in &cmds { println!("[{}] {:?} -> {}", TAG, cmd, execute(cmd)); }
}
