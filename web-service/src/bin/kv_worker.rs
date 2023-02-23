use kvweb;
use std::{thread, time};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // KVFinder path
    kv_path: String,
    // path to save jobs
    job_path: String,
}

fn main() {
    println!("KVFinder Worker started");
    let args = Cli::from_args();
    let config = kvweb::worker::Config {
        kv_path: args.kv_path,
        job_path: args.job_path,
    };
    loop {
        // get the next job from queue. If there is not a job to process then wait 5 seconds.
        let r = kvweb::worker::get_job();
        match r {
            Ok(j) => {
                let id = j.id;
                // process a job and submit the results (update job at the queue).
                match kvweb::worker::process(j, &config) {
                    Err(e) => println!("Error processing: {}", e),
                    Ok(output) => match kvweb::worker::submit_result(id, output) {
                        Ok(id) => println!("Job processed successfully: {}", id),
                        Err(e) => println!("Error submitting result to queue: {}", e),
                    },
                }
            }
            //no job to process
            Err(_) => thread::sleep(time::Duration::from_secs(5)),
        }
    }
}
