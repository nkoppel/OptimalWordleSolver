async function get_solution(url, compressed) {
    var xmlhttp = new XMLHttpRequest()

    var promise = new Promise(resolve => {
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 && this.status == 200) {
                if (compressed) {
                    resolve(decompress(this.responseText))
                } else {
                    resolve(parse(this.responseText))
                }
            }
        };
    })

    xmlhttp.open("GET", url, true)
    xmlhttp.send()
    return await promise
}

function filter_solution(solution, depth, hint) {
    return solution.filter(elem => elem[depth * 2 + 1] === hint)
}

function average_turns(solution) {
    var total_turns = solution.reduce((sum, elem) => sum + elem.length / 2, 0)

    return total_turns / solution.length
}

function parse(text) {
    var list = text.split("\n").filter(e => e.length != 0)

    for (var i = 0; i < list.length; i++) {
        list[i] = list[i].split(" ")
    }

    return list
}

function compress(solution) {
    var out = "0 " + solution[0].slice(0, solution[0].length - 1).join(" ") + "\n"

    for (var i = 1; i < solution.length; i++) {

        // works because when index is out of bounds,
        // solution[j] === undefined
        var j = 0
        while (solution[i - 1][j] === solution[i][j]) {j++}

        out += (j / 2 + 1 | 0) + " " + solution[i].slice(j, solution[i].length - 1).join(" ") + "\n"
    }

    return out
}

function decompress(text) {
    var lines = text.split("\n")
    var out = []

    for (var i = 0; i < lines.length; i++) {
        line = lines[i].trim().split(" ")
        j = parseInt(line[0])

        out.push((out[i - 1] || [])
            .slice(0, j * 2 - 1)
            .concat(line.slice(1))
            .concat(["ggggg"]))
    }

    return out
}
