# Rust BSQ

This program was one of the first ones I did in my first year of university, where we had to implement it in C. While learning Rust, I thought it would be cool to reimplement these first programs, and indeed it was a good idea. I discovered a lot of new methods that make the development of these programs easier.

## How it Works

1. The first part of the program generates a random map with obstacles with a certain density. The map is a bitfield, where a bit is set to 1 if it is an obstacle. I used bitfields to use less space, so the size will be `x * y / 8 + ((x * y % 8) ? 1 : 0)` instead of `x * y`.

2. The generated map is then passed to the algorithm, which generates the same map but with integers. From there, it clones the map to retain the positions of the obstacles, since the program will then do some computations to facilitate and accelerate the search for the biggest square.

3. After all the information has been found and stored, the final part of the algorithm is to actually find the biggest square. It will be a classic brute-force approach but with a smarter way to check if a square can enter a given coordinate, all with the help of the information gathered in step 2.

## Implementation Details

This program is implemented in Rust, and bitfields are used to optimize the space used to store the generated maps.

## Resources

- Algorithm explained on Stack Overflow: https://stackoverflow.com/questions/20335427/most-efficient-algorithm-to-find-the-biggest-square-in-a-two-dimension-map
- Algorithm explanation: https://docs.google.com/document/d/19pHCD433tYsvAor0WObxa2qusAjKdx96kaf3z5I8XT8/edit
- https://stackoverflow.com/questions/1726632/dynamic-programming-largest-square-block

## Conclusion

This program was a great opportunity to reimplement a classic algorithm while learning new techniques in Rust. The use of bitfields was an interesting optimization, and the program performs well even with large maps.
