ARROW BOT
=========

Queue up arrow commands, press Run, and guide the robot to the goal.

HOW TO PLAY
-----------
1. The red tile is the START. The orange tile is the END (the goal).
   The dark blue tiles are rocks. The robot cannot go through them.
2. Click a box in the bottom row to add a command.
   Each click changes it: up -> right -> down -> left -> empty.
3. Press the green play button (top-left) or Space to run the program.
   The robot follows the boxes from left to right, one step at a time.
4. The robot turns green when it reaches the END.
   It turns red if it hits a rock, leaves the map, or runs out of commands.

CONTROLS
--------
Left click on a box ..... change the command
Right click on a box .... clear the command
Play button / Space ..... run the program
Circle arrow button / R . new random map, clears the program
S ....................... save the program
L ....................... load the saved program
Esc ..................... quit

Your program is saved automatically to robot_code.txt in the folder
where you start the game.

LINUX
-----
If the game does not start, make the file executable first:

    chmod +x arrow-bot
    ./arrow-bot

Enjoy!