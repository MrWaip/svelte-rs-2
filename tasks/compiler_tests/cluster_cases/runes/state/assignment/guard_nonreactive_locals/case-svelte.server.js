import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function update() {
		let x, y;
		[x, y] = [1, 2];
		return x + y;
	}
	$$renderer.push(`<button>0</button>`);
}
