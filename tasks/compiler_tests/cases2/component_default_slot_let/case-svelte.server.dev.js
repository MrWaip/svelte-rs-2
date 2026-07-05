App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import List from "./List.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		List($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { item }) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 6, 1);
				$$renderer.push(`${$.escape(item)}</p>`);
				$.pop_element();
			} }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
