import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let arr = [1];
	function update() {
		let x = [a] = arr;
		return x;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
