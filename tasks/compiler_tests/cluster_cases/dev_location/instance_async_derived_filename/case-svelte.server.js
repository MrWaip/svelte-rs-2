import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { p } = $$props;
	var total;
	var $$promises = $$renderer.run([async () => total = await $.async_derived(() => p)]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(total())));
	$$renderer.push(`</p>`);
}
