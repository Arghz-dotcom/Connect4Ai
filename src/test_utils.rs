

fn batchTest(filesTest:&mut Vec<&str>, solver:alphaBetaSolverInterface, nbReadRows:u32) {
    let path = require("path");
    for filename in filesTest {
        let absPath = path.resolve("./src/utils/data/" + filename);
        let data = readFileSync(absPath, "utf-8");
        let splitted = data.split(/\r?\n/);
        nbReadRows = Math.min(nbReadRows, splitted.length);
        let totalNodeCount = 0, totalTimeMs = 0
        
        for(let indexRow = 0; indexRow < nbReadRows; indexRow++) {            
            let [toPlaySequence, expectedScore] = splitted[indexRow].split(' ')
            solver.playSequence(toPlaySequence)
            let score = solver.solve(25)
            totalTimeMs += solver.elapsedTimeMs
            totalNodeCount += solver.nodeCount

            expect(parseInt(expectedScore)-score).toBe(0)
        }

        let meanTime = (totalTimeMs/nbReadRows).toFixed(2)
        let meanNbPos = (totalNodeCount/nbReadRows).toFixed(2)
        let kpos = (totalNodeCount/totalTimeMs).toFixed(2)
        console.log("Mean time: %d, mean nb pos: %d, k pos/s: %d", meanTime, meanNbPos, kpos)
    }