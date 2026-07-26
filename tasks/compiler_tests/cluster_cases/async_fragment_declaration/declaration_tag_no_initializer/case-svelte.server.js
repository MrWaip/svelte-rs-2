import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	let a;
	let b;
	var promises = $$renderer.run([async () => a = (await $.save(Promise.resolve(n)))(), () => {}]);
	$$renderer.push(`<p>`);
	$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(a)));
	$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
	$$renderer.push(`</p> <button>go</button>`);
}
