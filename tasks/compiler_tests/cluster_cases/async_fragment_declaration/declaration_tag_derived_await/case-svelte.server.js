import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		let g;
		var promises = $$renderer.run([async () => g = await $.async_derived(async () => (await $.save(Promise.resolve(n)))())]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(g())));
		$$renderer.push(`</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
