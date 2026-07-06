import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	let z = 0;
	let arr = [
		1,
		2,
		3
	];
	function update() {
		[x, ...{z = 26}] = arr;
	}
	$$renderer.push(`<button>${$.escape(x)}${$.escape(z)}</button>`);
}
