import * as wasm from "hello-wasm-pack";
import * as sim from "lib-simulation-wasm";

const viewport = document.getElementById("viewport");

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;

var stop = false;
var frameCount = 0;
var fps, fpsInterval, startTime, now, then, elapsed;

function startAnimating(fps) {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    animate();
}

function rotate(cx, cy, x, y, radians) {
    radians -= Math.PI/2;
    let cos = Math.cos(radians);
    let sin = Math.sin(radians);
    let nx = (cos * (x - cx)) + (sin * (y - cy)) + cx;
    let ny = (cos * (y - cy)) - (sin * (x - cx)) + cy;
    return [nx, ny];
}

CanvasRenderingContext2D.prototype.drawFish = 
    function (x, y, size, rotation) {
        rotation = -rotation
        ctxt.beginPath();
        var [nx, ny] = rotate(x, y, x + (0.6 * size), y, rotation);
        ctxt.moveTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0 * size), y + (0.3 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0.3 * size), y + (0.05 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0.55 * size), y + (0.25 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0.55 * size), y - (0.25 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0.3 * size), y - (0.05 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x - (0 * size), y - (0.3 * size), rotation);
        ctxt.lineTo(nx, ny);
        [nx, ny] = rotate(x, y, x + (0.6 * size), y, rotation);
        ctxt.lineTo(nx, ny);

        ctxt.fillStyle = "rgb(140, 170, 238)";
        ctxt.fill();
    }

CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        this.beginPath();

        this.arc(x, y, radius, 0, 2.0 * Math.PI);

        this.fillStyle = "rgb(166, 218, 149)";
        this.fill();
    };

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext("2d");

ctxt.scale(viewportScale, viewportScale);
ctxt.fillStyle = "rgb(0, 0, 0)"

const simulation = new sim.Simulation();

startAnimating(30);

function animate() {

    // request another frame

    requestAnimationFrame(animate);

    // calc elapsed time since last loop

    now = Date.now();
    elapsed = now - then;

    // if enough time has elapsed, draw the next frame

    if (elapsed > fpsInterval) {

        // Get ready for next frame by setting then=now, but also adjust for your
        // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
        then = now - (elapsed % fpsInterval);

        redraw();

    }
}


function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

    simulation.step();

    for (const agent of simulation.world().agents) {
        ctxt.drawFish(agent.x * viewportWidth, agent.y * viewportHeight, 0.04 * viewportWidth, agent.rotation);
    }

    for (const food of simulation.world().foods) {
        ctxt.drawCircle(food.x * viewportWidth, food.y * viewportHeight, 0.005 * viewportWidth);
    }
}