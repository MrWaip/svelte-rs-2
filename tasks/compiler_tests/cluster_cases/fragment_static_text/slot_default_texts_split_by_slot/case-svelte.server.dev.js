App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		C($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<!---->foobar`);
			}),
			$$slots: {
				default: true,
				s: ($$renderer) => {
					$$renderer.push(`<x slot="s">`);
					$.push_element($$renderer, "x", 1, 6);
					$$renderer.push(`y</x>`);
					$.pop_element();
				}
			}
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
