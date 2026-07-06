import * as $ from "svelte/internal/server";
import { SvelteSet } from "svelte/reactivity";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const set = new SvelteSet();
		$$renderer.push(`<button>add</button>`);
	});
}
