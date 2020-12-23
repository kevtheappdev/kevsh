
pub enum State {
    FG,
    BG,
    UNDEF,
    ST,
}

pub struct Job {
    pid: u32,
    jid: u32,
    state: State,
}

pub struct JobManager {
    jobs: Vec<Job>,
    nextJid: u32,
}

impl JobManager {

    pub fn init() -> JobManager {
        JobManager {
            jobs: Vec::new(),
            nextJid: 0,
        }   
    }

    pub fn addJob(&mut self, pid: u32, state: State) {
        let job = Job {pid: pid, jid: self.nextJid, state: state};
        self.nextJid += 1;
        self.jobs.push(job);
    }

    pub fn deleteJob(&mut self, pid: u32) {
        self.jobs.retain(|job| {
            job.pid != pid
        });
    }

    pub fn listJobs(&self) {
        println!("listing jobs");
    }

}