import fs from "fs/promises";

// part 1
(async () => {
    const scores =
    
    await fs.readFile("input.txt", "utf-8")
    .then((data) => data.split("\n").slice(0, -1))
    .then((data) => 
        data
        .map(e => e.split(" "))

        // convert A, B, C and X, Y, Z to 0, 1, 2
        .map(e => [e[0].charCodeAt(0) - 65, e[1].charCodeAt(0) - 88])

        .map(e => ({
            played: e[1],   // the move played
            outcome: (
                (
                    ((3 + e[1] - e[0]) % 3) // to get the outcome of the game (0 = draw, 1 = win, 2 = loss)
                    * 3
                ) + 3
            ) % 9,  // the rest of operations is to map (0, 1, 2) to (3, 6, 0)
        }))
        .map(e => e.outcome + e.played + 1)
        .reduce((a, b) => a + b, 0) // sum the points
    )

    console.log(scores);
});


// 1D array of the table of operations for the 2nd part (enemy + win/lose/draw -> move to play)
/**
 * Function table in 1D packed array
 * (enemy + win/lose/draw -> move to play)
 * 
 * 0 = lose
 * 1 = draw
 * 2 = win
 * 
 * | o | 0 | 1 | 2 |
 * |---|---|---|---|
 * | 0 | 2 | 0 | 1 |
 * | 1 | 0 | 1 | 2 |
 * | 2 | 1 | 2 | 0 |
 * 
 * @TODO this could probably be done in a more elegant, mathematical way
 */

// 2d representation of the 1d lookup table
const lookup2D = [
    [3, 4, 8],
    [1, 5, 9],
    [2, 6, 7],
];


// part 2
(async () => {
    const scores =
    
    await fs.readFile("input.txt", "utf-8")
    .then((data) => 
        data
        .split("\n")
        .slice(0, -1)
        .map(e => e.split(" "))
        .map(e => [e[0].charCodeAt(0) - 65, e[1].charCodeAt(0) - 88])
        .map(e => lookup2D[e[0]][e[1]])
        .reduce((a, b) => a + b, 0)
    )

    console.log(scores);
})();