import * as sim from 'lib-simulation-wasm';

const simulation = new sim.Simulation();
const world = simulation.world();

const viewport = document.getElementById('viewport');