import fs from "fs/promises";


(async () => {

    const pairs = await fs.readFile("input.txt", "utf-8").then(d => d.split("\n").slice(0, -1).map(e => e.split(",").map(e => e.split("-").map(e => parseInt(e)))));

    const contained = pairs.filter(e => e[0][0] <= e[1][0] && e[0][1] >= e[1][1] || e[0][0] >= e[1][0] && e[0][1] <= e[1][1]);
    const overlapping = pairs.filter(e => Math.max(e[0][0], e[1][0]) <= Math.min(e[0][1], e[1][1]))

    console.log(`Contained: ${contained.length}`);
    console.log(`Overlapping: ${overlapping.length}`);

})();