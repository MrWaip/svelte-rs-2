import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { cond, aGet, aSet, bGet, bSet } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (cond) {
				$$renderer.push("<!--[0-->");
				var bind_get = () => aGet();
				var bind_set = aSet;
				Child($$renderer, {
					get value() {
						return bind_get();
					},
					set value($$value) {
						bind_set($$value);
					}
				});
			} else {
				$$renderer.push("<!--[-1-->");
				var bind_get_1 = () => bGet();
				var bind_set_1 = bSet;
				Child($$renderer, {
					get value() {
						return bind_get_1();
					},
					set value($$value) {
						bind_set_1($$value);
					}
				});
			}
			$$renderer.push(`<!--]-->`);
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
	});
}
