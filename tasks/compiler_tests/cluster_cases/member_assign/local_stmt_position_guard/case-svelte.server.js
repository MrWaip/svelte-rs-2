import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cache = {};
	function set(item) {
		cache[item.id] = item;
	}
	$$renderer.push(`<button>go</button>`);
}
