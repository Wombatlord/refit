use std::{
    env::args,
    fs::{read_dir, rename, File},
    io::{self, BufRead, BufReader, Write},
};

fn main() {
    let chosen_palette = args().nth(1).expect("No palette selected.");

    let configs_path = match File::open("./refit.conf") {
        Ok(f) => BufReader::new(f)
            .lines()
            .next()
            .expect("Error reading file")
            .expect("Empty file"),
        Err(_) => {
            let mut refit = File::create("refit.conf").expect("Failed to create refit.conf");
            println!("Created refit.conf file. Please enter path to your starship.toml directory.");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            refit
                .write(input.as_bytes())
                .expect("Failed to write to refit.conf.");
            input.to_string().trim().to_string()
        }
    };
    let e = &format!("starship.toml not found.\n{} AAAA", configs_path);
    let starship = File::open(format!("{}/starship.toml", configs_path)).expect(e);
    let first_line = BufReader::new(starship)
        .lines()
        .next()
        .expect("empty file")
        .expect("failed to read line");
    let palette_name = first_line.split(" ").collect::<Vec<&str>>();

    match rename(
        format!("{}/starship.toml", configs_path),
        format!("{}/starship.{}.toml", configs_path, palette_name[1]),
    ) {
        Ok(_) => {}
        Err(e) => eprint!("{e}"),
    };

    let configs_dir = read_dir(configs_path.clone()).unwrap();

    for file in configs_dir {
        match file {
            Ok(f) => {
                if f.file_name().to_str().unwrap().trim()
                    == format!("starship.{}.toml", chosen_palette)
                {
                    rename(
                        format!("{}/starship.{}.toml", configs_path, chosen_palette),
                        format!("{}/starship.toml", configs_path),
                    )
                    .expect("Failed to rename starship.%.toml");
                } else {
                    continue;
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
