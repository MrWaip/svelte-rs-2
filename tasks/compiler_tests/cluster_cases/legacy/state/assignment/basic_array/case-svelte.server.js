import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let b = 0;
	function update() {
		[a, b] = [1, 2];
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
