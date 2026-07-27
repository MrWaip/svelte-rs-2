import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cache = {};
	async function go() {
		const value = cache.value ??= await get_value();
	}
	async function get_value() {
		return 42;
	}
	$$renderer.push(`<button>go</button>`);
}
