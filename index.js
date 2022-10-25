import { Application } from './runtime.js';

export class Spline {
    canvas = null;

    constructor(canvas) {
        this.canvas = canvas;
    }

    run(url) {
        if (this.canvas != null) {
            console.log(this.canvas);
            
            const app = new Application(this.canvas);
            app.load(url);
        }
    }
}