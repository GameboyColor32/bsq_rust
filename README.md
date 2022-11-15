# Rust BSQ
This program was one of the first one I did in my first year in university. We had to make it in C.
While learning Rust I thought it would be cool to develop those first programs again. Indeed it was a good
idea, I discovered a lot of new methods I didn't know about when I was starting that actually make the
development of the programs easier.

How does it work?
=====================
Step 1
---------------------
The first part of the program is generating a random map with obstacles with a certain density.
The map is a bitfield, a bit is set at 1 if it is an obstacle. I used btiflields in order to use less space, so
x * y size will be x * y / 8 + ((x * y % 8) ? 1 : 0) instead.

Step 2
---------------------
Now we have to pass the generated map to the actual algorithm. What it will do first is to generate the same
map but with integers. From there, it will clone the map, to retain the positions of the obstacles, since the program
will then do some computations to facilitate and accelerate the search of the biggest square.

Step 3
---------------------
After all the information has been found and stored, the final part of the algorithm is to actually find the biggest square.
It will be a classic brute-force but with a smarter way to check if a squ


are can enter a given coordinate, all that
with the help of the information gathered in step 2.

Ressources
-----------------------------------
-   Algorithm explained on Stackoverflow: https://stackoverflow.com/questions/20335427/most-efficient-algorithm-to-find-the-biggest-square-in-a-two-dimension-map
-   https://stackoverflow.com/questions/1726632/dynamic-programming-largest-square-block
-   Algorithm explanation: https://docs.google.com/document/d/19pHCD433tYsvAor0WObxa2qusAjKdx96kaf3z5I8XT8/edit
