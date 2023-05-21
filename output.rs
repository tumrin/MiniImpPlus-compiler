fn main(){
let mut REPLAY;
while(REPLAY == "yes"{
let mut PLAYER_ONE;
let mut PLAYER_ONE_GESTURE;
let mut PLAYER_TWO;
let mut PLAYER_TWO_GESTURE;
println!("Insert name of player 1:");
let mut PLAYER_ONE = String::new();
std::io::stdin().read_line(&mut PLAYER_ONE).unwrap();
println!("Insert name of player 2:");
let mut PLAYER_TWO = String::new();
std::io::stdin().read_line(&mut PLAYER_TWO).unwrap();
println!("Starting game with players:");
println!("{PLAYER_ONE}");
println!("{PLAYER_TWO}");
println!("Player 1, choose a gesture (rock/paper/scissors): ");
let mut PLAYER_ONE_GESTURE = String::new();
std::io::stdin().read_line(&mut PLAYER_ONE_GESTURE).unwrap();
println!("Player 2, choose a gesture (rock/paper/scissors): ");
let mut PLAYER_TWO_GESTURE = String::new();
std::io::stdin().read_line(&mut PLAYER_TWO_GESTURE).unwrap();
if(PLAYER_TWO_GESTURE == PLAYER_ONE_GESTURE){
println!("Draw")}
;
if(PLAYER_TWO_GESTURE == "rock"&&(PLAYER_ONE_GESTURE == "scissors"{
println!("Player 2 wins")}
;
if(PLAYER_TWO_GESTURE == "rock"&&(PLAYER_ONE_GESTURE == "paper"{
println!("Player 1 wins")}
;
if(PLAYER_TWO_GESTURE == "scissors"&&(PLAYER_ONE_GESTURE == "paper"{
println!("Player 2 wins")}
;
if(PLAYER_TWO_GESTURE == "scissors"&&(PLAYER_ONE_GESTURE == "rock"{
println!("Player 1 wins")}
;
if(PLAYER_TWO_GESTURE == "paper"&&(PLAYER_ONE_GESTURE == "scissors"{
println!("Player 1 wins")}
;
if(PLAYER_TWO_GESTURE == "paper"&&(PLAYER_ONE_GESTURE == "rock"{
println!("Player 2 wins")}
;
println!("Do you want to play again?")let mut REPLAY = String::new();
std::io::stdin().read_line(&mut REPLAY).unwrap();
}
}
}
