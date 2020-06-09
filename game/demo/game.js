function setup() {
  createCanvas(770, 700);
  player = new Player(createVector(500, 500));
}

function draw() {
  fill(0);
  rect(0, height - 5, width, 5);
  player.render();
}

let player;