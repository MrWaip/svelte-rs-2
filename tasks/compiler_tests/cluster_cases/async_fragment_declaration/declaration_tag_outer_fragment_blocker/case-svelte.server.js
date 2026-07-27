import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		let outer;
		var promises = $$renderer.run([async () => outer = await $.async_derived(async () => (await $.save(Promise.resolve(n)))())]);
		{
			let inner;
			var promises_1 = $$renderer.run([() => promises[0], () => inner = $.derived(() => `v${outer()}`)]);
			$$renderer.push(`<div><span>`);
			$$renderer.async([promises_1[1]], ($$renderer) => $$renderer.push(() => $.escape(inner())));
			$$renderer.push(`</span></div>`);
		}
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
