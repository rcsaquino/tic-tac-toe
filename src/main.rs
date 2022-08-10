use std::io::{self, Write};

fn main() {
    // Setup variables
    let mut board: [String; 8] = [
        String::from("---"),
        String::from(""),
        String::from("1 | 2 | 3"),
        String::from("---------"),
        String::from("4 | 5 | 6"),
        String::from("---------"),
        String::from("7 | 8 | 9"),
        String::from(""),
    ];
    let mut round: u8 = 0;
    let mut game_over: bool = false;
    let mut player_picks: [Vec<u8>; 2] = [vec![], vec![]];

    for row in &board {
        println!("{}", row);
    }

    // Game loop
    while !game_over {
        round += 1;
        let playing: &str = if round % 2 != 0 { "X" } else { "O" };
        let player_i: usize = if round % 2 != 0 { 0 } else { 1 };

        let mut temp_board = board.clone();

        let mut valid_answer = false;

        while !valid_answer {
            let answer = prompt(&format!("{} turn: ", playing));
            for (i, row) in board.iter().enumerate() {
                if row.contains(&answer) {
                    temp_board[i] = row.replace(&answer, playing);
                    player_picks[player_i].push(answer.parse::<u8>().unwrap());
                    valid_answer = true;
                }
            }
            if !valid_answer {
                println!("Invalid position!");
            }
        }

        board = temp_board;

        for row in &board {
            println!("{}", row);
        }

        // Check win conditions
        
        // Horizontal
        if (player_picks[player_i].contains(&1)
            && player_picks[player_i].contains(&2)
            && player_picks[player_i].contains(&3))
            || (player_picks[player_i].contains(&4)
                && player_picks[player_i].contains(&5)
                && player_picks[player_i].contains(&6))
            || (player_picks[player_i].contains(&7)
                && player_picks[player_i].contains(&8)
                && player_picks[player_i].contains(&9))

            // Vertical
            || (player_picks[player_i].contains(&1)
                && player_picks[player_i].contains(&4)
                && player_picks[player_i].contains(&7))
            || (player_picks[player_i].contains(&2)
                && player_picks[player_i].contains(&5)
                && player_picks[player_i].contains(&8))
            || (player_picks[player_i].contains(&3)
                && player_picks[player_i].contains(&6)
                && player_picks[player_i].contains(&9))

            // Diagonal
            || (player_picks[player_i].contains(&1)
                && player_picks[player_i].contains(&5)
                && player_picks[player_i].contains(&9))
            || (player_picks[player_i].contains(&3)
                && player_picks[player_i].contains(&5)
                && player_picks[player_i].contains(&7))
        {
            let answer = prompt(&format!("{} won! Play again? (y/n): ", playing));
            if &answer == "y" {
                main()
            }
            game_over = true
        } else {
            // Check tie condition
            let combined = [&player_picks[0][..], &player_picks[1][..]].concat();

            let mut tied = true;
            for x in 1..=9 as u8 {
                if !combined.contains(&x) {
                    tied = false;
                }
            }
            if tied {
                let answer = prompt(&format!("It's a tie! Play again? (y/n): "));
                if &answer == "y" {
                    main()
                }
                game_over = true
            }
        }
    }
}

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
