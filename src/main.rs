use std::env;


//allows for comparing enums with ==
#[derive(PartialEq, Eq)]
enum resp_type {
    yes,
    no,
    unknown,
}

fn main() {
    
    //get arguments as tuple and split
    //to two vars
    let parsed_args  = get_args();
    let project_name: String = parsed_args.0;
    let project_dir: String = parsed_args.1;

    let mut args_ok = String::new();    

    println!("Project Name = {} and Project Dir = {} is this correct? (Y) (N)", project_name, project_dir);

    //read user input from command line
    std::io::stdin().read_line(&mut args_ok).unwrap();

    println!("You wrote: {}", args_ok);


    let mut resp = resp_type::unknown;
    
    if args_ok.trim() == "Y" || args_ok.trim() == "y" {
        resp = resp_type::yes;
    }else if args_ok.trim() == "N" || args_ok.trim() == "n"{
        resp = resp_type::no;
    }else{
        resp = resp_type::unknown;
    }

    let mut new_args_if_no = String::new();

    if resp == resp_type::no || resp == resp_type::unknown {

        while resp == resp_type::no || resp == resp_type::unknown {
            
            new_args_if_no = String::new();

            if resp == resp_type::no {
                println!("Gotcha please enter a new project name followed by a space followed by directory");
                std::io::stdin().read_line(&mut new_args_if_no).unwrap();
                println!("Ok looks like {} is this correct (Y) (N) \n", new_args_if_no);
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

