import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function fetchValue() {
		return 5;
	}
	var x;
	var $$promises = $$renderer.run([async () => x = await $.async_derived(fetchValue)]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(x())));
	$$renderer.push(`</p>`);
}
