import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { promise } = $$props;
	var value;
	var $$promises = $$renderer.run([async () => value = await promise]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(value)));
	$$renderer.push(`</p>`);
}
