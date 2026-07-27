import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	var d, e;
	var $$promises = $$renderer.run([async () => d = await $.async_derived(() => Promise.resolve(n)), async () => e = await $.async_derived(() => Promise.resolve(n + 1))]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		if (d()) {
			$$renderer.push("<!--[0-->");
			let v;
			var promises = $$renderer.run([() => Promise.all([$$promises[0], $$promises[1]]), () => v = d() + e()]);
			$$renderer.push(`<p>`);
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(v)));
			$$renderer.push(`</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <button>go</button>`);
}
