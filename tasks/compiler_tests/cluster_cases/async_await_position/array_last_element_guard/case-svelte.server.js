import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	function wrap(values) {
		return values.length;
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.push(async () => $.escape(wrap([1, await delay(x)])));
}
