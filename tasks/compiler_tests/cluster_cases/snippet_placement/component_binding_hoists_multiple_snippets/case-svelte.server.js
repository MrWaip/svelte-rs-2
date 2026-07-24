import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = void 0, items } = $$props;
		function a($$renderer, p) {
			$$renderer.push(`<span>${$.escape(items)} ${$.escape(p)}</span>`);
		}
		function b($$renderer, q) {
			$$renderer.push(`<em>${$.escape(items)} ${$.escape(q)}</em>`);
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
					a($$renderer, 1);
					$$renderer.push(`<!---->`);
					b($$renderer, 2);
					$$renderer.push(`<!---->`);
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
