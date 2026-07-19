import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function setup(id) {
		const entry = globalThis.__cache[id] ??= {};
		return entry;
	}
	$$renderer.push(`<button>go</button>`);
}
