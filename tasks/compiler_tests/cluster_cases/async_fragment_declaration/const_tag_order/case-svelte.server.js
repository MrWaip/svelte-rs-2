import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		const before = "sync";
		let awaited;
		let after;
		var promises = $$renderer.run([async () => awaited = (await $.save(Promise.resolve(n)))(), () => after = awaited + 1]);
		$$renderer.push(`<p>sync`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(awaited)));
		$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(after)));
		$$renderer.push(`</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
