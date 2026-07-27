import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		let a;
		let b;
		var promises = $$renderer.run([async () => a = await $.async_derived(async () => (await $.save(Promise.resolve(n)))()), () => b = a() * 2]);
		$$renderer.push(`<span>`);
		$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
		$$renderer.push(`</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
