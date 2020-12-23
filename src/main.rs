use std::io::{stdin, stdout};
use std::io::Write;
use std::process::{Command, Stdio};

mod jobs;



struct KSH {
    jobManager: jobs::JobManager,
}

impl KSH {

    fn execute(&mut self, exec: &str, args: std::str::SplitWhitespace, isBG: bool) {
        let mut child = Command::new(exec).args(args)
                                          .stdin(Stdio::piped())
                                          .stdout(Stdio::piped()).spawn().unwrap();
        println!("command: {}", exec);
        self.jobManager.addJob(child.id(), jobs::State::FG);

        if !isBG {
            let ecode = child.wait().expect("failed to wait on child");
        }
    }


    fn eval(&mut self, input: String) {
        let line = input.trim();
        if line.is_empty() {
            return
        }

        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        let isBG = false;

        let args = parts;
        match command {
            "quit" => std::process::exit(0),
            "jobs" => self.jobManager.listJobs(),
            _ => self.execute(command, args, isBG),
        }
    }

    fn init() -> KSH {
        // set up signal handlers
        ctrlc::set_handler(move || {
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        // TODO: need signal handlers for SIGCHLD, etc.

        return KSH {
            jobManager: jobs::JobManager::init(),
        }
    }

}

fn main() {
    let mut ksh = KSH::init();

    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(value) => ksh.eval(input),
            _ => continue,
        }
    }
}
