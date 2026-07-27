import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	$$renderer.push(`<p>${$.escape(count)}</p> <button>inc</button> <button>double</button>`);
}
