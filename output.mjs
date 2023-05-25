import * as readline from 'node:readline/promises';
import { stdin as input, stdout as output } from 'node:process';
{
                        let REPLAY = (true); while (REPLAY === (true)) {
                                                let PLAYER_ONE = (""); let PLAYER_ONE_GESTURE = (""); let PLAYER_TWO = (""); let PLAYER_TWO_GESTURE = (""); let INPUT = (""); console.log(("Insert name of player 1:")); const rl_PLAYER_ONE = readline.createInterface({ input, output });
                                                PLAYER_ONE = await rl_PLAYER_ONE.question('');
                                                rl_PLAYER_ONE.close(); console.log(("Insert name of player 2:")); const rl_PLAYER_TWO = readline.createInterface({ input, output });
                                                PLAYER_TWO = await rl_PLAYER_TWO.question('');
                                                rl_PLAYER_TWO.close(); console.log(("Starting game with players:")); console.log((PLAYER_ONE)); console.log((PLAYER_TWO)); console.log(("Choose a gesture (rock/paper/scissors) for the following player: ")); console.log((PLAYER_ONE)); const rl_PLAYER_ONE_GESTURE = readline.createInterface({ input, output });
                                                PLAYER_ONE_GESTURE = await rl_PLAYER_ONE_GESTURE.question('');
                                                rl_PLAYER_ONE_GESTURE.close(); console.log(("Choose a gesture (rock/paper/scissors) for the following player: ")); console.log((PLAYER_TWO)); const rl_PLAYER_TWO_GESTURE = readline.createInterface({ input, output });
                                                PLAYER_TWO_GESTURE = await rl_PLAYER_TWO_GESTURE.question('');
                                                rl_PLAYER_TWO_GESTURE.close(); if (PLAYER_TWO_GESTURE === (PLAYER_ONE_GESTURE)) { console.log(("Draw")); } if (PLAYER_TWO_GESTURE === ("rock") && PLAYER_ONE_GESTURE === ("scissors")) { console.log((PLAYER_TWO)); console.log((" wins")); } if (PLAYER_TWO_GESTURE === ("rock") && PLAYER_ONE_GESTURE === ("paper")) { console.log((PLAYER_ONE)); console.log((" wins")); } if (PLAYER_TWO_GESTURE === ("scissors") && PLAYER_ONE_GESTURE === ("paper")) { console.log((PLAYER_TWO)); console.log((" wins")); } if (PLAYER_TWO_GESTURE === ("scissors") && PLAYER_ONE_GESTURE === ("rock")) { console.log((PLAYER_ONE)); console.log((" wins")); } if (PLAYER_TWO_GESTURE === ("paper") && PLAYER_ONE_GESTURE === ("scissors")) { console.log((PLAYER_ONE)); console.log((" wins")); } if (PLAYER_TWO_GESTURE === ("paper") && PLAYER_ONE_GESTURE === ("rock")) { console.log((PLAYER_TWO)); console.log((" wins")); } console.log(("Do you want to play again?")); const rl_INPUT = readline.createInterface({ input, output });
                                                INPUT = await rl_INPUT.question('');
                                                rl_INPUT.close(); if (INPUT === ("yes")) { REPLAY = (true); } else { REPLAY = (false); }
                        }
} 
