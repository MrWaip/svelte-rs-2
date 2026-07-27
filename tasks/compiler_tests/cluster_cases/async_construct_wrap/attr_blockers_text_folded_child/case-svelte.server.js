import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var msg;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => msg = "hi"]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(msg)));
	$$renderer.push(`</p>`);
}
