var matchfiles = [
    "example-matches/ex-01.json",
    "example-matches/ex-02.json",
    "example-matches/ex-03.json",
    "example-matches/ex-04.json",
    "example-matches/ex-05.json",
    "example-matches/ex-06.json",
    "example-matches/ex-07.json",
    "example-matches/ex-08.json",
    "example-matches/ex-09.json",
    "example-matches/ex-10.json"
];

var matches = [];
var loaded = 0;

var currentMatch = 0;
var currentGame = 0;
var maxTurns = 0;
var currentTurn = 0;

function clearPlayfield() {
    let playfield = document.getElementById("playfield");
    for (i in playfield.children) {
        let child = playfield.children[i];
        child.innerHTML = "";
    }
}

var turnLen = 100;

function doTurn() {
    console.log("turn");
    if (currentTurn < maxTurns) {
        clearPlayfield();
        let match = matches[currentMatch];
        let game = match.games[currentGame];
        let turn = game.turns[currentTurn];
        let p1 = turn.state.p1;
        let p2 = turn.state.p2;
        let p1pos = p1.pos;
        let p2pos = p2.pos;
        let p1energy = p1.energy;
        let p2energy = p2.energy;
        let playfield = document.getElementById("playfield");
        let p1div = playfield.children[p1pos];
        let p2div = playfield.children[p2pos];
        p1div.innerHTML = '<img src="images/ferris-fencing-gold.svg">';
        p2div.innerHTML = '<img src="images/ferris-fencing-blue.svg">';

        
        let p1ediv = document.getElementById("p1energy");
        let p2ediv = document.getElementById("p2energy");
        let p1ep = p1energy / 30000 * 100;
        let p1eps = p1ep.toString() + "%";
        let p2ep = p2energy / 30000 * 100;
        let p2eps = p2ep.toString() + "%";

        p1ediv.style["width"] = p1eps;
        p2ediv.style["width"] = p2eps;
        
        currentTurn += 1;
        console.log("set timeout");
        window.setTimeout(doTurn, turnLen);
    } else {
        doEnd();
        currentTurn = 0;
        window.setTimeout(doTurn, turnLen);
    }
}

function runCurrent() {
    let match = matches[currentMatch];
    let game = match.games[currentGame];
    maxTurns = game.turns.length;
    doTurn();
}

function startMatches() {
    console.log("starting");

    clearPlayfield();

    runCurrent();
}

function matchLoaded() {
    var match = JSON.parse(this.responseText);
    matches.push(match);


    loaded += 1;

    if (loaded == matchfiles.length) {
        startMatches()
    }
}

function loadMatches() {
    for (index in matchfiles) {
        var file = matchfiles[index];

        var oReq = new XMLHttpRequest();
        oReq.addEventListener("load", matchLoaded);
        oReq.open("GET", file);
        oReq.send();        
    }
}


loadMatches();
