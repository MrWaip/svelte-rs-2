import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([async () => a = await Promise.resolve(1)]);
	$$renderer.push(`<!--[-->`);
	{
		let b;
		var promises = $$renderer.run([async () => b = (await $.save(Promise.resolve(2)))()]);
		$$renderer.async_block([promises[0], $$promises[0]], ($$renderer) => {
			if (b + a > 0) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>yes</p>`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
