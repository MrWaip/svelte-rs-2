import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		let s;
		var promises = $$renderer.run([async () => s = (await $.save(Promise.resolve(n)))()]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(s)));
		$$renderer.push(`</p> <button>bump</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
