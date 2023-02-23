use std::env;
use text_colorizer::*;
use std::process::{Command, Stdio};
use std::str;



//TODO create regex that makes sure the provided directory is only letters, nums, 
//and forward slashes

//for even friendlier user experience, let the user choose what is allowed in a directory...
//

//allows for comparing enums with ==
#[derive(PartialEq, Eq)]
enum resp_type {
    yes,
    no,
    unknown,
}

fn main() {
    println!("\n");
    
    let mut project_name = String::new();
    let mut project_dir = String::new();
    
    //get arguments as tuple and split
    //to two vars
    let parsed_args  = get_args();
    project_name = parsed_args.0;
    project_dir = parsed_args.1;

    let mut args_ok = String::new();    

    println!("{} = {} and {} = {} is this correct? (Y) (N)", "Project Name".blue(), project_name.purple(), "Project Dir".green(), project_dir.purple());

    //read user input from command line
    std::io::stdin().read_line(&mut args_ok).unwrap();

    println!("You wrote: {}", args_ok);


    //check if user was good with their arguments passed
    let mut resp = resp_type::unknown;
    
    if args_ok.trim() == "Y" || args_ok.trim() == "y" {
        resp = resp_type::yes;
    }else if args_ok.trim() == "N" || args_ok.trim() == "n"{
        resp = resp_type::no;
    }else{
        resp = resp_type::unknown;
    }

    //for getting new arguments from user
    //if they aren't satisfied with current args
    let mut new_args_if_no = String::new();

    //resolve no or unknown answers
    if resp == resp_type::no || resp == resp_type::unknown {

        while resp == resp_type::no || resp == resp_type::unknown {
            
            new_args_if_no = String::new();

            if resp == resp_type::no {
                println!("Gotcha please enter a new project name followed by a space followed by directory");
                std::io::stdin().read_line(&mut new_args_if_no).unwrap();

                
                let mut found_words = false;
                
                //make sure the user truly entered one word followed by a space
                while found_words == false{
                    let split_count = new_args_if_no.split(" ");
                    if split_count.count() !=  2 {
                        println!("\n");
                        println!("{} please enter one word without spaces followed by a space, followed by another word without spaces \n", "Error".red());
                        new_args_if_no = String::new();
                        std::io::stdin().read_line(&mut new_args_if_no).unwrap();
                        found_words = false;
                    }else{
                        let split = new_args_if_no.split(" ");
                        let mut index = 0;
                        for word in split{
                            if index == 0 {
                                project_name = word.to_string();
                                index = index + 1;
                            }else{
                                project_dir = word.to_string();
                            }
                        }
                        found_words = true;
                    }
                    
                    

                   
                }

                println!("{} = {} and {} = {} is this correct? (Y) (N)", "Project Name".blue(), project_name.purple(), "Project Dir".green(), project_dir.purple());
            }else{
                println!("Sorry didn't catch that please enter (Y) or (N) \n");
            }

        
            args_ok = String::new();            

            std::io::stdin().read_line(&mut args_ok).unwrap();
        

            if args_ok.trim() == "Y" || args_ok.trim() == "y" {
                resp = resp_type::yes;
            }else if args_ok.trim() == "N" || args_ok.trim() == "n"{
                resp = resp_type::no;
            }else{
                resp = resp_type::unknown;
            }

        }
    }

    if resp == resp_type::yes{
        println!("Great Moving On :)  \n");
    }else if resp == resp_type::no {
        println!("Gotcha please enter project name followed by a space \n");        
    }else{
        println!("is uknown");
    }


    

    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(["/C", "echo hello Windows is terrible why do you use it?"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg(format!("cd {} && cargo new {}", project_dir, project_name))
    //             .output()
    //             .expect("failed to execute process")
    // };

    // let hello = output.stdout;

    
    println!("Created package, would you like to push to a new github repo?");

    let output2 = Command::new("sh")
                .arg("-c")
                .arg(format!("cd {}/{} && ls", project_dir, project_name))
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process");

    let echo_out = output2.stdout.expect("Failed to open echo stdout");

    let mut git_child = Command::new("echo")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let output3 = git_child.wait_with_output().expect("Failed to wait on git");
    
    // assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());

    let s = match str::from_utf8(&output3.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", s);

}


fn get_args() -> (String, String) {

    //get command line args
    let args: Vec<String> = env::args().skip(1).collect();
    
    //make sure right number of args are present
    if args.len() != 2 {
        eprintln!("Error: got {} args expected 2 args", args.len());
        eprintln!("cmdline_crate - create detailed rust projects from command line");
        eprintln!("Usage: cmdline_crate <package name>, <project directory>");
        std::process::exit(1);
    }

    //apparently clone copies an object
    //not sure if this is right at the 
    //moment will need to dive deeper    
    //return tuple
    (args[0].clone(), args[1].clone())
}

