import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function go() {
		let o = { a: 0 };
		[o.a] = [1];
		return o.a;
	}
	$$renderer.push(`<button>0</button>`);
}
