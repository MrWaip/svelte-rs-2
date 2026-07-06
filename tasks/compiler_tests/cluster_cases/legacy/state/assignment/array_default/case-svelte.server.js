import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let arr = [];
	function update() {
		[a = 5] = arr;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
