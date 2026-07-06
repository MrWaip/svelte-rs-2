import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let obj = { x: 0 };
	let arr = [1, 2];
	function update() {
		[obj.x, a] = arr;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
