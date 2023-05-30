fn main() {
    let mut REPLAY = true;
    while REPLAY == true {
        let mut PLAYER_ONE = "";
        let mut PLAYER_ONE_GESTURE = "";
        let mut PLAYER_TWO = "";
        let mut PLAYER_TWO_GESTURE = "";
        let mut INPUT = "";
        println!("{}", "Insert name of player 1:");
        let mut PLAYER_ONE = String::new();
        std::io::stdin().read_line(&mut PLAYER_ONE).unwrap();
        PLAYER_ONE = PLAYER_ONE.trim().to_string();
        println!("{}", "Insert name of player 2:");
        let mut PLAYER_TWO = String::new();
        std::io::stdin().read_line(&mut PLAYER_TWO).unwrap();
        PLAYER_TWO = PLAYER_TWO.trim().to_string();
        println!("{}", "Starting game with players:");
        println!("{}", PLAYER_ONE);
        println!("{}", PLAYER_TWO);
        println!(
            "{}",
            "Choose a gesture (rock/paper/scissors) for the following player: "
        );
        println!("{}", PLAYER_ONE);
        let mut PLAYER_ONE_GESTURE = String::new();
        std::io::stdin().read_line(&mut PLAYER_ONE_GESTURE).unwrap();
        PLAYER_ONE_GESTURE = PLAYER_ONE_GESTURE.trim().to_string();
        println!(
            "{}",
            "Choose a gesture (rock/paper/scissors) for the following player: "
        );
        println!("{}", PLAYER_TWO);
        let mut PLAYER_TWO_GESTURE = String::new();
        std::io::stdin().read_line(&mut PLAYER_TWO_GESTURE).unwrap();
        PLAYER_TWO_GESTURE = PLAYER_TWO_GESTURE.trim().to_string();
        if PLAYER_TWO_GESTURE == PLAYER_ONE_GESTURE {
            println!("{}", "Draw");
        }
        if PLAYER_TWO_GESTURE == "rock" && PLAYER_ONE_GESTURE == "scissors" {
            println!("{}", PLAYER_TWO);
            println!("{}", " wins");
        }
        if PLAYER_TWO_GESTURE == "rock" && PLAYER_ONE_GESTURE == "paper" {
            println!("{}", PLAYER_ONE);
            println!("{}", " wins");
        }
        if PLAYER_TWO_GESTURE == "scissors" && PLAYER_ONE_GESTURE == "paper" {
            println!("{}", PLAYER_TWO);
            println!("{}", " wins");
        }
        if PLAYER_TWO_GESTURE == "scissors" && PLAYER_ONE_GESTURE == "rock" {
            println!("{}", PLAYER_ONE);
            println!("{}", " wins");
        }
        if PLAYER_TWO_GESTURE == "paper" && PLAYER_ONE_GESTURE == "scissors" {
            println!("{}", PLAYER_ONE);
            println!("{}", " wins");
        }
        if PLAYER_TWO_GESTURE == "paper" && PLAYER_ONE_GESTURE == "rock" {
            println!("{}", PLAYER_TWO);
            println!("{}", " wins");
        }
        println!("{}", "Do you want to play again?");
        let mut INPUT = String::new();
        std::io::stdin().read_line(&mut INPUT).unwrap();
        INPUT = INPUT.trim().to_string();
        if INPUT == "yes" {
            REPLAY = true;
        } else {
            REPLAY = false;
        }
    }
}

