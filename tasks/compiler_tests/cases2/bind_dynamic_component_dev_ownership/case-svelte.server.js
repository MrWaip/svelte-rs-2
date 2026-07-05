import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = void 0 } = $$props;
		let Comp = A;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (Comp) {
				$$renderer.push("<!--[-->");
				Comp($$renderer, {
					get value() {
						return value;
					},
					set value($$value) {
						value = $$value;
						$$settled = false;
					}
				});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		$.bind_props($$props, { value });
	});
}
