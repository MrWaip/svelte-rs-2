App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Component($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$.prevent_snippet_stringification(children);
				function children($$renderer, { with_prop }) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<!---->txt ${$.escape(with_prop)}`);
				}
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 1, 11);
				$$renderer.push(`</span>`);
				$.pop_element();
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
