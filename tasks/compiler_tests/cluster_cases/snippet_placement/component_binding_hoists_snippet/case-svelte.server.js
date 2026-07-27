import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = void 0, items } = $$props;
		function foo($$renderer, a) {
			$$renderer.push(`<span>${$.escape(items)} ${$.escape(a)}</span>`);
		}
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			Child($$renderer, {
				get ref() {
					return ref;
				},
				set ref($$value) {
					ref = $$value;
					$$settled = false;
				},
				children: ($$renderer) => {
					foo($$renderer, 2);
				},
				$$slots: { default: true }
			});
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		$.bind_props($$props, { ref });
	});
}
