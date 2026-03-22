use std::{fs, str::FromStr};


pub fn read_array<T: PartialOrd + Copy + FromStr>(path: &str) -> Vec<T>{


    let content = fs::read_to_string(path);

    let arrray = match content {
        Ok(value) =>{
            value
                .trim()
                .split(',')
                .map(|s| s.trim()
                .parse::<T>())
                .flatten() 
                .collect()
        }

        Err(e) => {
            eprintln!("Error reading file: {}", e);
            (1..=10).rev()
                .filter_map(|n| T::from_str(&n.to_string()).ok())
                .collect()
        }
    };

    arrray
}