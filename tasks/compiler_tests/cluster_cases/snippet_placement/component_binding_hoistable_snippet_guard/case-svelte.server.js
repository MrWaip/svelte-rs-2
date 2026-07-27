import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { CONST } from "x";
function foo($$renderer, a) {
	$$renderer.push(`<span>${$.escape(CONST)} ${$.escape(a)}</span>`);
}
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = void 0 } = $$props;
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
					foo($$renderer, 1);
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
