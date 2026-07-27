App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = void 0, items } = $$props;
		function a($$renderer, p) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 6, 15);
			$$renderer.push(`${$.escape(items)} ${$.escape(p)}</span>`);
			$.pop_element();
		}
		function b($$renderer, q) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<em>`);
			$.push_element($$renderer, "em", 7, 15);
			$$renderer.push(`${$.escape(items)} ${$.escape(q)}</em>`);
			$.pop_element();
		}
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$.prevent_snippet_stringification(a);
			$.prevent_snippet_stringification(b);
			Child($$renderer, {
				get ref() {
					return ref;
				},
				set ref($$value) {
					ref = $$value;
					$$settled = false;
				},
				children: $.prevent_snippet_stringification(($$renderer) => {
					a($$renderer, 1);
					$$renderer.push(`<!---->`);
					b($$renderer, 2);
					$$renderer.push(`<!---->`);
				}),
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
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
