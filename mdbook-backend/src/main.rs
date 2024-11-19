use std::{fs, io, path::Path};


// I want to console out the 

fn main() -> io::Result<()> {

    println!("helllooo");

    let dir = std::env::current_dir().unwrap();

    
    let root = dir.ancestors().nth(1).unwrap_or_else(|| Path::new("/"));
    println!("Root of the current folder system: {:?}", root);

    for entry in fs::read_dir(root.join("src"))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("File: {:?}", path);
        } else if path.is_dir() {
            println!("Directory: {:?}", path);
        }
    }
    // let mut entries = fs::read_dir(dir)?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    // println!("hello me {:?}", entries );

    Ok(())
    // let path = "../../src/new_file.txt";
        // let mut file = match File::create(path) {
        //     Ok(file) => file,
        //     Err(e) => {
        //         eprintln!("Failed to create file: {}", e);
        //         return;
        //     }
        // };

        // match file.write_all(b"Hello, world!") {
        //     Ok(_) => println!("File created successfully"),
        //     Err(e) => eprintln!("Failed to write to file: {}", e),
        // }
}
