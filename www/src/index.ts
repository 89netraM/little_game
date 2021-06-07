import "./index.scss";
import little_game from "little_game";

let canvas: HTMLCanvasElement;

window.addEventListener(
	"load",
	async () => {
		canvas = document.getElementById("canvas") as HTMLCanvasElement;
		updateCanvasSize();
		await little_game();
	},
	true
);

window.addEventListener("resize", () => updateCanvasSize(), true);

function updateCanvasSize(): void {
	const canvasRect = canvas.getBoundingClientRect();
	canvas.width = canvasRect.width;
	canvas.height = canvasRect.height;
}
