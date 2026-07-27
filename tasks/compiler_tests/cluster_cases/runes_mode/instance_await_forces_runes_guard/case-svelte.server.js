import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	var value;
	var $$promises = $$renderer.run([async () => value = await Promise.resolve(1)]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(value)));
	$$renderer.push(` ${$.escape(count)}</p> <button>inc</button>`);
}
