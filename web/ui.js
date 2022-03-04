// TODO: have all input act on "hints", and update interface based on it's value

const ROWS = 6;
const COLUMNS = 5;

async function init() {
    for (var i = 0; i < ROWS; i++) {
        var row = d3.select("#board").append("div")

        row.classed("row", true)

        for (var j = 0; j < COLUMNS; j++) {
            var tile = row.append("div")

            tile.classed("tile blank-tile", true)

            tile.classed("tile_row" + i + " tile_column" + j, true)
        }
    }

    d3.select("#keyboard").style("display", "flex")

    await set_solution("solutions/full_solution_compressed.txt", true)
    render()
}

init()

function get_tile(r, c) {
    return d3.select(".tile_row" + r + ".tile_column" + c)
}

function style_element(elem, color) {
    if (!elem.node()) {
        return;
    }

    elem.classed("grey-tile green-tile yellow-tile blank-tile blank-text-tile", false)

    switch (color) {
        case "grey": case "_":
            elem.classed("grey-tile", true)
            break
        case "green": case "g":
            elem.classed("green-tile", true)
            break
        case "yellow": case "y":
            elem.classed("yellow-tile", true)
            break
        case "blank-text": case "t":
            elem.classed("blank-text-tile", true)
            break
        case "blank": case " ":
            elem.classed("blank-tile", true)
            break
    }
}

function set_tile(r, c, text, color) {
    var tile = get_tile(r, c)

    style_element(tile, color)
    tile.html(text)
}

var Hints = Array(6).fill("")
var Solution = []

async function set_solution(url, compressed) {
    window.Solution = await get_solution(url, compressed) 
    window.WorkingSolution = Solution
}

function render() {
    var solution = Solution;
    var words = []
    var hints = Hints.slice()

    words.push(solution[0][0])

    for (var i = 0; i < 6; i++) {
        if (Hints[i] && Hints[i].length == 5) {
            solution = filter_solution(solution, i, Hints[i])

            if (solution[0]) {
                words.push(solution[0][(i + 1) * 2])
            } else {
                hints[i + 1] = ""
                window.Hints[i + 1] = ""
            }
        } else {
            if (hints[i - 1] == "ggggg") {
                hints[i] = ""
                window.Hints[i] = ""
            } else if (solution.length == 1) {
                hints[i] = "ggggg"
                window.Hints[i] = ""
            } 

            break
        }
    }

    elem = d3.selectAll(".tile")
    style_element(elem, "blank")
    elem.text("")

    for (var r = 0; r < words.length; r++) {
        for (var c = 0; c < 5; c++) {
            var style = (hints[r] || [])[c] || (words[r] ? "w" : " ")
            var text  = (words[r] || [])[c] || "";

            set_tile(r, c, text.toUpperCase(), style)
        }
    }
}

function resetHints() {
    window.Hints = Array(6).fill("")
    render()
}

function addToHints(chr) {
    for (var i = 0; i < Hints.length; i++) {
        if (Hints[i].length < 5) {
            window.Hints[i] += chr
            render()
            return
        }
    }
}

function deleteFromHints() {
    for (var i = Hints.length - 1; i >= 0; i--) {
        if (Hints[i].length > 0) {
            window.Hints[i] = Hints[i].substring(0, Hints[i].length - 1)
            render()
            return
        }
    }
}

document.addEventListener("keydown",  function (e) {
    console.log(e.key)

    switch (e.key) {
        case "b":
        case " ":
        case "_": addToHints("_"); break
        case "g": addToHints("g"); break
        case "y": addToHints("y"); break
        case "Backspace": deleteFromHints(); break
        case "Escape": resetHints(); break
    }
})

d3.select("#green-key" ).node().addEventListener("click", e => addToHints("g"))
d3.select("#yellow-key").node().addEventListener("click", e => addToHints("y"))
d3.select("#grey-key"  ).node().addEventListener("click", e => addToHints("_"))

d3.select("#delete-key").node().addEventListener("click", e => deleteFromHints())
d3.select("#clear-key" ).node().addEventListener("click", e => resetHints())
