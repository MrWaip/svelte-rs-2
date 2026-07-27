import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	var d;
	var $$promises = $$renderer.run([async () => d = await $.async_derived(() => Promise.resolve(n))]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		if (d()) {
			$$renderer.push("<!--[0-->");
			let a;
			let b;
			var promises = $$renderer.run([
				() => $$promises[0],
				() => a = d() + 1,
				() => b = a + 1
			]);
			$$renderer.push(`<p>`);
			$$renderer.async([promises[2]], ($$renderer) => $$renderer.push(() => $.escape(b)));
			$$renderer.push(`</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <button>go</button>`);
}
