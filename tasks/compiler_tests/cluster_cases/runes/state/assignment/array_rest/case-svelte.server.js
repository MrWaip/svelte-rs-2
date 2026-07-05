import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let b = 0;
	let arr = [
		1,
		2,
		3
	];
	function update() {
		[a, ...b] = arr;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
