import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function load() {
		count = await Promise.resolve(1);
	}
	$$renderer.push(`<p>${$.escape(count)}</p> <button>load</button>`);
}
