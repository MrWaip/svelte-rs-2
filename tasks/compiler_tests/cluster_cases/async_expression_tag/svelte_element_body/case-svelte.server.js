import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$.element($$renderer, "p", void 0, () => {
		$$renderer.push(async () => $.escape(await delay(x)));
	});
}
