import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	if (n) {
		$$renderer.push("<!--[0-->");
		let a;
		let b;
		let c;
		var promises = $$renderer.run([async () => a = (await $.save(Promise.resolve(n)))(), () => {
			b = $.derived(() => a * 2);
			c = a + 1;
		}]);
		$$renderer.push(`<span>`);
		$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b())));
		$$renderer.push(` `);
		$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(c)));
		$$renderer.push(`</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
