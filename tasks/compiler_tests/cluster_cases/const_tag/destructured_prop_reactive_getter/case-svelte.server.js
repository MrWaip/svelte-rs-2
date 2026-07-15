import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		if (data) {
			$$renderer.push("<!--[0-->");
			const simpleReactive = data.foo;
			const { destr } = { destr: 1 };
			const simpleStatic = 5;
			Child($$renderer, {
				a: simpleReactive,
				b: destr,
				c: simpleStatic
			});
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
}
