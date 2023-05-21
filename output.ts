
require('node:readline/promises');

require('node:process');

const rl = readline.createInterface({ stdin, stdout });{ 
let REPLAY = false;
while( REPLAY ){ 
let PLAYER_ONE = "";
let PLAYER_ONE_GESTURE = "";
let PLAYER_TWO = "";
let PLAYER_TWO_GESTURE = "";
let INPUT = "";
console.log("Insert name of player 1:");
let PLAYER_ONE = rl.question();
console.log("Insert name of player 2:");
let PLAYER_TWO = rl.question();
console.log("Starting game with players:");
console.log("PLAYER_ONE");
console.log("PLAYER_TWO");
console.log("Player 1, choose a gesture (rock/paper/scissors): ");
let PLAYER_ONE_GESTURE = rl.question();
console.log("Player 2, choose a gesture (rock/paper/scissors): ");
let PLAYER_TWO_GESTURE = rl.question();
if (PLAYER_TWO_GESTURE === PLAYER_ONE_GESTURE ){ 
console.log("Draw")
};
if (PLAYER_TWO_GESTURE === "rock" && PLAYER_ONE_GESTURE === "scissors" ){ 
console.log("2 wins")
};
if (PLAYER_TWO_GESTURE === "rock" && PLAYER_ONE_GESTURE === "paper" ){ 
console.log("1 wins")
};
if (PLAYER_TWO_GESTURE === "scissors" && PLAYER_ONE_GESTURE === "paper" ){ 
console.log("2 wins")
};
if (PLAYER_TWO_GESTURE === "scissors" && PLAYER_ONE_GESTURE === "rock" ){ 
console.log("1 wins")
};
if (PLAYER_TWO_GESTURE === "paper" && PLAYER_ONE_GESTURE === "scissors" ){ 
console.log("1 wins")
};
if (PLAYER_TWO_GESTURE === "paper" && PLAYER_ONE_GESTURE === "rock" ){ 
console.log("2 wins")
};
console.log("Do you want to play again?");
let INPUT = rl.question();
if (INPUT === "yes" ){ 
REPLAY = true
}else{ 
REPLAY = false
};

}
}unknown