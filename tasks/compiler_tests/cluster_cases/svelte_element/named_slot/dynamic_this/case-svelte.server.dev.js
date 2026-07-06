App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = $.fallback($$props["tag"], "h1");
		Foo($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$.validate_dynamic_element_tag(() => tag);
				$.validate_void_dynamic_element(() => tag);
				$.push_element($$renderer, tag, 7, 1);
				$.element($$renderer, tag, void 0, () => {
					$$renderer.push(`This is default slot`);
				});
				$.pop_element();
			}),
			$$slots: {
				default: true,
				other: ($$renderer) => {
					$.validate_dynamic_element_tag(() => tag);
					$.validate_void_dynamic_element(() => tag);
					$.push_element($$renderer, tag, 8, 1);
					$.element($$renderer, tag, () => {
						$$renderer.push(` slot="other"`);
					}, () => {
						$$renderer.push(`This is other slot`);
					});
					$.pop_element();
				}
			}
		});
		$.bind_props($$props, { tag });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
