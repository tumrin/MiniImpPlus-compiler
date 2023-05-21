
const readline = require('node:readline');

const { stdin: input, stdout: output } = require('node:process');

const rl = readline.createInterface({ input, output });{ 
let REPLAY = true;
while( REPLAY ){ 
let PLAYER_ONE = "";
let PLAYER_ONE_GESTURE = "";
let PLAYER_TWO = "";
let PLAYER_TWO_GESTURE = "";
let INPUT = "";
console.log("Insert name of player 1:");
rl.on("line", (answer: string) => {
                        PLAYER_ONE = answer;
                        console.log("hello bois");
                      });;
console.log("Insert name of player 2:");
rl.on("line", (answer: string) => {
                        PLAYER_TWO = answer;
                        console.log("hello bois");
                      });;
console.log("Starting game with players:");
console.log(PLAYER_ONE);
console.log(PLAYER_TWO);
console.log("Player 1, choose a gesture (rock/paper/scissors): ");
rl.on("line", (answer: string) => {
                        PLAYER_ONE_GESTURE = answer;
                        console.log("hello bois");
                      });;
console.log("Player 2, choose a gesture (rock/paper/scissors): ");
rl.on("line", (answer: string) => {
                        PLAYER_TWO_GESTURE = answer;
                        console.log("hello bois");
                      });;
if (PLAYER_TWO_GESTURE === PLAYER_ONE_GESTURE ){ 
console.log("Draw")
};
if (PLAYER_TWO_GESTURE === "rock" && PLAYER_ONE_GESTURE === "scissors" ){ 
console.log(PLAYER_TWO);
console.log(" wins")
};
if (PLAYER_TWO_GESTURE === "rock" && PLAYER_ONE_GESTURE === "paper" ){ 
console.log(PLAYER_ONE);
console.log(" wins")
};
if (PLAYER_TWO_GESTURE === "scissors" && PLAYER_ONE_GESTURE === "paper" ){ 
console.log(PLAYER_TWO);
console.log(" wins")
};
if (PLAYER_TWO_GESTURE === "scissors" && PLAYER_ONE_GESTURE === "rock" ){ 
console.log(PLAYER_ONE);
console.log(" wins")
};
if (PLAYER_TWO_GESTURE === "paper" && PLAYER_ONE_GESTURE === "scissors" ){ 
console.log(PLAYER_ONE);
console.log(" wins")
};
if (PLAYER_TWO_GESTURE === "paper" && PLAYER_ONE_GESTURE === "rock" ){ 
console.log(PLAYER_TWO);
console.log(" wins")
};
console.log("Do you want to play again?");
rl.on("line", (answer: string) => {
                        INPUT = answer;
                        console.log("hello bois");
                      });;
if (INPUT === "yes" ){ 
REPLAY = true
}else{ 
REPLAY = false
};

}
}