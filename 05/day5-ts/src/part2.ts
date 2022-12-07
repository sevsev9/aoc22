import fs from "fs/promises";

(async () =>  {
    const file = await fs.readFile("input.txt", "utf-8").then((data) => data.split("\n"));
    
    const head = file.splice(0, file.indexOf("") - 1);
    const moves = file.splice(2, file.length - 3).map(e => e.split(" ")).map(e => [parseInt(e[1]), parseInt(e[3]) - 1, parseInt(e[5]) - 1]);

    const towers = head.map(e => [...e.match(/.{1,4}/g)!.values()]).reduce( (arr: Array<Array<string>>, val: string[]) => {
        val.forEach( (v, i) => {
            if (!arr[i]) {
                arr[i] = new Array<string>();
            }

            const letter = v.match(/[A-Z]{1}/g);

            if (letter) {
                arr[i].push(letter[0]);
            }
        });

        return arr;
    }, new Array<Array<string>>());

    towers.forEach( (tower, idx) => {
        towers[idx] = tower.reverse();
        console.log(tower.join(" - "));
    })
    

    for (const move of moves) {
        process.stdout.write(`${move[1]}{${move[0]}} -> ${move[2]} => `)
        towers[move[2]].push(
            ...towers[move[1]]
            .splice(
                towers[move[1]].length - move[0],
                move[0]
            )
        );
        console.log(`${towers[move[2]].join(" - ")}`);
    }

    for (const tower of towers) {
        console.log(tower.join(" - "));
    }    
})();