import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> <p><span>`);
	$$renderer.push(async () => $.escape((await $.save(delay(x)))()));
	$$renderer.push(`</span></p>`);
}
