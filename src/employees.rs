use std::io;

pub fn collect_employees() {
    let mut names: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read line failed");

        let line = line.trim().to_string();

        if line == "x" {
            names.sort();
            for name in names.iter() {
                println!("{}", name);
            }

            break;
        }

        names.push(line)
    }
}
