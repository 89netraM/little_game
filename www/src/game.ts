export class Game {
	private readonly canvas: HTMLCanvasElement;
	private movement: JsVector2 = new JsVector2();
	private hasFocus: boolean = false;

	public constructor(canvas: HTMLCanvasElement) {
		this.canvas = canvas;

		this.hide_cursor = this.hide_cursor.bind(this);
		this.onMouseMove = this.onMouseMove.bind(this);
		this.get_cursor_movement = this.get_cursor_movement.bind(this);
		this.onPointerLock = this.onPointerLock.bind(this);
		this.get_focus = this.get_focus.bind(this);

		this.canvas.addEventListener("mousemove", this.onMouseMove, true);
		document.addEventListener("pointerlockchange", this.onPointerLock, true);
	}

	public hide_cursor(hidden: boolean): void {
		if (hidden) {
			this.canvas.requestPointerLock();
			this.movement = new JsVector2();
			this.hasFocus = true;
		}
		else {
			document.exitPointerLock();
			this.hasFocus = false;
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

	private onPointerLock(): void {
		this.hasFocus = document.pointerLockElement === this.canvas;
	}

	public get_focus(): boolean {
		return this.hasFocus;
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
