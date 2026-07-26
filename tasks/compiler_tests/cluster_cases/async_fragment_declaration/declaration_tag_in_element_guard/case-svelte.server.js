import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	{
		let a;
		var promises = $$renderer.run([async () => a = (await $.save(Promise.resolve(n)))()]);
		$$renderer.push(`<div><p>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(a)));
		$$renderer.push(`</p></div>`);
	}
	$$renderer.push(` <button>go</button>`);
}
