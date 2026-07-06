import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function load() {
		return 1;
	}
	$$renderer.push(`<p>`);
	$$renderer.push(async () => $.escape((await $.save(load()))()));
	$$renderer.push(`</p>`);
}
