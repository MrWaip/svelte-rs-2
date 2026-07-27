App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = void 0, items } = $$props;
		function foo($$renderer, a) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 6, 17);
			$$renderer.push(`${$.escape(items)} ${$.escape(a)}</span>`);
			$.pop_element();
		}
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$.prevent_snippet_stringification(foo);
			Child($$renderer, {
				get ref() {
					return ref;
				},
				set ref($$value) {
					ref = $$value;
					$$settled = false;
				},
				children: $.prevent_snippet_stringification(($$renderer) => {
					foo($$renderer, 2);
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
