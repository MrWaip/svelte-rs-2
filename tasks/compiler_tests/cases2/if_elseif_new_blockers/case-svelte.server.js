import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a, b;
	var $$promises = $$renderer.run([async () => a = await first_fetch(), async () => b = await second_fetch()]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		if (a) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>a</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.async_block([$$promises[1]], ($$renderer) => {
				if (b) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<p>b</p>`);
				} else {
					$$renderer.push("<!--[-1-->");
					$$renderer.push(`<p>fallback</p>`);
				}
			});
			$$renderer.push(`<!--]-->`);
		}
	});
	$$renderer.push(`<!--]-->`);
}
