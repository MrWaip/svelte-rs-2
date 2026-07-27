import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> <!--[-->`);
	$.slot($$renderer, $$props, "default", { value: delay(x) }, null);
	$$renderer.push(`<!--]-->`);
}
