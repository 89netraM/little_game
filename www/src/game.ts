export class Game {
	private readonly canvas: HTMLCanvasElement;
	private movement: JsVector2 = new JsVector2();

	public constructor(canvas: HTMLCanvasElement) {
		this.canvas = canvas;

		this.onMouseMove = this.onMouseMove.bind(this);
		this.canvas.addEventListener("mousemove", this.onMouseMove, true);
	}

	public hide_cursor(hidden: boolean): void {
		if (hidden) {
			this.canvas.requestPointerLock();
			this.movement = new JsVector2();
		}
		else {
			document.exitPointerLock();
		}
	}

	private onMouseMove(e: MouseEvent): void {
		this.movement.add(e.movementX, e.movementY);
	}

	public get_cursor_movement(): JsVector2 {
		try {
			return this.movement;
		}
		finally {
			this.movement = new JsVector2();
		}
	}
}

class JsVector2 {
	public x: number = 0;
	public y: number = 0;

	public add(x: number, y: number): void {
		this.x += x;
		this.y += y;
	}
}
