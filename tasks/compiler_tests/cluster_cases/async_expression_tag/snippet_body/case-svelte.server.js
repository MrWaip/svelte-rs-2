import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	function row($$renderer) {
		$$renderer.push(`<!---->`);
		$$renderer.push(async () => $.escape(await delay(x)));
	}
	$$renderer.push(`<button>inc</button> `);
	row($$renderer);
	$$renderer.push(`<!---->`);
}
