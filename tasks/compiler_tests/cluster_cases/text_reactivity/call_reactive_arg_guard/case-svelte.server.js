import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function fn(x) {
		return x;
	}
	$$renderer.push(`<p>v ${$.escape(fn(count))}</p> <button>+</button>`);
}
