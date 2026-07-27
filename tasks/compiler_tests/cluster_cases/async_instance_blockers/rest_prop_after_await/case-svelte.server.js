import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function inc() {
			c++;
		}
		var x, a, b, c, rest;
		var $$promises = $$renderer.run([async () => x = await Promise.resolve(1), () => ({a, b = 2, c = 3, $$slots, $$events, ...rest} = $$props)]);
		$$renderer.push(`<button>`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(x)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(c)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(JSON.stringify(rest))));
		$$renderer.push(`</button>`);
		$.bind_props($$props, { c });
	});
}
