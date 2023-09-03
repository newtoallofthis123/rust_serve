use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Rserver", author="Ishan Joshi", version, about="A Simple rust server", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false)]
    entry: Option<String>,
}

mod server;

fn main() {
    let args = Args::parse();
    if args.entry.is_none(){
        bunt::println!("No {$yellow}Entry Point{/$} specified");
        bunt::println!("Searching for {$yellow}index.html{/$} in current directory...");
        if std::path::Path::new("index.html").exists(){
            bunt::println!("{$green}Found index.html{/$}");
        }else{
            bunt::println!("{$red}No index.html found{/$}, generating a directory listing");
        }
    }
    else{
        if std::path::Path::new(&args.entry.clone().unwrap()).exists(){
            bunt::println!("{$green}Found {}{/$}", args.entry.unwrap());
        }else{
            bunt::println!("{$red}No {} found{/$}, generating a directory listing", args.entry.unwrap());
        }
    }
}
