import fs from "fs/promises";

(async () => {
    const start = performance.now();
    const file = await fs.readFile("input.txt", "utf-8");

    const inventory = file.split("\n").reduce((inv: number[][], line) => {
        line === "" ? inv.push([]) : inv[inv.length - 1].push(parseInt(line));
        return inv;
    }, [[]]).map(e => e.reduce((a, b) => a + b, 0)).sort((a, b) => a - b).reverse();

    const end = performance.now();
    console.log(`Time: ${(end - start).toFixed(2)}ms`);

    console.log(`Calories held by the top elf: ${inventory[0]}`);
    console.log(`Calories held by the top 3 elves: ${inventory[0] + inventory[1] + inventory[2]}`);
})();