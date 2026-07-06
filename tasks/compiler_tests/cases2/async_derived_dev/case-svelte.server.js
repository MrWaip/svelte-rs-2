import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var data;
	var $$promises = $$renderer.run([async () => data = await $.async_derived(() => fetch("/api"))]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data())));
	$$renderer.push(`</p>`);
}
