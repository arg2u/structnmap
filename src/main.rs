use std::env;
use std::fs::read_to_string;
use std::path::Path;

use structnmap::Data;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        let read_path = &args[1];
        if Path::new(read_path).exists() {
            let save_path = &args[2];
            if Path::new(save_path).exists() {
                let file = read_to_string(read_path).unwrap();
                let data = Data::build(file);
                data.generate(&save_path);
            } else {
                eprintln!("\nIncorrect output dir path.\n")
            }
        } else {
            eprintln!("\nIncorrect file path specified for reading.\n")
        }
    } else if args.len() > 1 {
        let arg = &args[1];
        if arg == "-h" || arg.contains("help") {
            println!("========================================");
            println!("Table must have this structure:");
            println!("========================================");
            println!("||===IP===||===PORT===||===PROTOCOL===||");
            println!("||10.0.0.0||   2222   ||      SSH     ||");
            println!("========================================");
            println!("Example:");
            println!("========================================");
            println!("structnmap <table_path> <output_dir_path>");
        }
    } else {
        eprintln!("\nIncorrect arguments.\nstructnmap -h for more information")
    }
}
