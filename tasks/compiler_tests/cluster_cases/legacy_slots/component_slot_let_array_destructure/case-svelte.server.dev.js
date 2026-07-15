App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Parent($$renderer, { $$slots: { item: ($$renderer, { foo: [a, b] }) => {
			Child($$renderer, {
				slot: "item",
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}`);
				}),
				$$slots: { default: true }
			});
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
