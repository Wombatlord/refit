use std::{
    env::{args, current_exe},
    fs::{read_dir, rename, File},
    io::{self, BufRead, BufReader, Write},
};

fn main() {
    let chosen_palette = args().nth(1).expect("No palette selected.");
    let current_loc = current_exe().unwrap();
    let cl = current_loc.parent().unwrap();
    let configs_path = match File::open(format!("{}/refit.conf", cl.to_str().unwrap())) {
        Ok(f) => BufReader::new(f)
            .lines()
            .next()
            .expect("Error reading file")
            .expect("Empty file"),
        Err(_) => {
            println!("No config found. Please enter path to your starship.toml directory.");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            let mut refit =
                File::create(format!("{}/refit.conf", input.clone().trim().to_string()))
                    .expect("Failed to create refit.conf");
            refit
                .write(input.as_bytes())
                .expect("Failed to write to refit.conf.");
            input.to_string().trim().to_string()
        }
    };
    let e = &format!("starship.toml not found at {}", configs_path);
    let starship = File::open(format!("{}/starship.toml", configs_path)).expect(e);
    let first_line = BufReader::new(starship)
        .lines()
        .next()
        .expect("empty file")
        .expect("failed to read line");
    let palette_name = first_line.split(" ").collect::<Vec<&str>>();

    let mut configs_dir = read_dir(configs_path.clone()).unwrap();
    let profile = configs_dir.find_map(|f| match f {
        Ok(fl) => {
            if fl.file_name().to_str().unwrap().trim()
                == format!("starship.{}.toml", chosen_palette)
            {
                Some(fl)
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    });


    if profile.is_some() {
        {
            match rename(
                format!("{}/starship.toml", configs_path),
                format!("{}/starship.{}.toml", configs_path, palette_name[1]),
            ) {
                Ok(_) => {}
                Err(e) => eprint!("{e}"),
            };

            rename(
                format!("{}/starship.{}.toml", configs_path, chosen_palette),
                format!("{}/starship.toml", configs_path),
            )
            .expect("Failed to rename starship.%.toml");
        } 
    } else {
        eprintln!("No profile matching {} found in {}", chosen_palette, configs_path)
    }
}
