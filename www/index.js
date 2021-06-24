import * as sim from 'lib-simulation-wasm';

const simulation = new sim.Simulation();
document.getElementById('train').onclick = () => {
    console.log(simulation.train());
};

/** @type {HTMLCanvasElement} */
const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation) {
    this.beginPath();
    const newX = Math.cos(rotation) * size * 1.5 + x;
    const newY = Math.sin(rotation) * size * 1.5 + y;
    this.moveTo(newX, newY);
    this.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
    );
    this.lineTo(
        x + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
    );
    this.lineTo(newX,newY);
    this.stroke();
    this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
};

CanvasRenderingContext2D.prototype.drewCircle = function (x, y, radius) {
    this.beginPath();
    this.arc(x, y, radius, 0, 2 * Math.PI);
    this.fillStyle = 'rgb(0, 255, 0)';
    this.fill();
}

const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

function redrew() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

    simulation.step();

    const world = simulation.world();

    for (const food of world.foods) {
        ctxt.drewCircle(food.x * viewportWidth, food.y * viewportHeight, (0.01 / 2.0) * viewportWidth);
    }

    for (const animal of world.animals) {
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation)
    }

    requestAnimationFrame(redrew);
}

redrew();
