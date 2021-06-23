import * as sim from 'lib-simulation-wasm';

const simulation = new sim.Simulation();
const world = simulation.world();
const animals = world.animals

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

const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

for (const animal of simulation.world().animals) {
    ctxt.drawTriangle(
        animal.x * viewportWidth,
        animal.y * viewportHeight,
        0.01 * viewportWidth,
        animal.rotation,
    );
}
