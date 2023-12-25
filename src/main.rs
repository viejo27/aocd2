use std::fs::read_to_string;

fn main() {
    let mut game_id: Option<u32> = Some(0);
    let mut sum: Option<u32> = Some(0);

    for line in read_to_string("./src/input.txt").unwrap().lines() {
        // println!("{}",line);
        let mut cube_cant: Option<u32> = Some(0);
        let mut word_iterator = line.split_whitespace();
        let mut red: Option<u32> = None;
        let mut green: Option<u32> = None;
        let mut blue: Option<u32> = None;
        let mut sum_game_id: bool = true;
        if word_iterator.next() == Some("Game") {
            let game = word_iterator.next().unwrap_or_default(); 
            if game.ends_with(":") {
                let id_game: Result<u32,_> = game.trim_end_matches(":").to_string().parse();
                match id_game {
                    Ok(id_game) => {
                        game_id = Some(id_game);
                        // println!("game id: {}",id_game);
                    }
                    Err(_) => {}
                }
            }
        }
        while let Some(word) = word_iterator.next() {
            let is_number: Result<u32,_> = word.to_string().parse();
            match is_number {
                Ok(cubes) => cube_cant = Some(cubes),
                Err(_err) => {
                    if word.ends_with(",") {
                        let cube_colour = word.trim_end_matches(",");
                        match cube_colour {
                            "red" => {
                                red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                            },
                            "green" => {
                                green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            "blue" => {
                                blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            _ => {}
                        }
                    }
                    if word.ends_with(";") {
                        let cube_colour = word.trim_end_matches(";");
                        match cube_colour {
                            "red" => {
                                red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                            },
                            "green" => {
                                green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            "blue" => {
                                blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            _ => {}
                        };
                        if sum_game_id && red <= Some(12) && green <= Some(13) && blue <= Some(14) {
                            red = None;
                            green = None;
                            blue = None;
                        } else {
                            sum_game_id = false;
                        }
                    }
                    if word == "red" {
                        red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                        if sum_game_id && red <= Some(12) && green <= Some(13) && blue <= Some(14) {
                            red = None;
                            green = None;
                            blue = None;
                        } else {
                            sum_game_id = false;
                        }
                    }
                    if word == "green" {
                        green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                        if sum_game_id && red <= Some(12) && green <= Some(13) && blue <= Some(14) {
                            red = None;
                            green = None;
                            blue = None;
                        } else {
                            sum_game_id = false;
                        }
                    }
                    if word == "blue" {
                        blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                        if sum_game_id && red <= Some(12) && green <= Some(13) && blue <= Some(14) {
                            red = None;
                            green = None;
                            blue = None;
                        } else {
                            sum_game_id = false;
                        }
                    }
                }
            }
        }
        if sum_game_id {
            sum = Some(sum.unwrap_or(0) + game_id.unwrap_or(0));
            // println!("game id is valid");
        }
    }
    println!("total is {:?}",sum);
}
