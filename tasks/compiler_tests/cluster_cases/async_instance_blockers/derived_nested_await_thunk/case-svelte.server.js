import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function g() {
		return 1;
	}
	async function f(v) {
		return v;
	}
	var x;
	var $$promises = $$renderer.run([async () => x = await $.async_derived(async () => await f((await $.save(g()))()))]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(x())));
	$$renderer.push(`</p>`);
}
