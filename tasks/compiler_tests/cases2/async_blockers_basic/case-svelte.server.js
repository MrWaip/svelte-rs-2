import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	var data, y;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => y = data.value]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(y)));
	$$renderer.push(`</p>`);
}
