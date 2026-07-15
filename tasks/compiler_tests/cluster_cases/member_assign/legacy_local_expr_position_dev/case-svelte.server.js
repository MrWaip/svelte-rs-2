import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cache = {};
	function fill(items) {
		items.forEach((item) => cache[item.id] = item);
	}
	$$renderer.push(`<button>go</button>`);
}
