App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = $$props["name"];
		Outer($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				{
					$$renderer.push(`hello ${$.escape(name)}`);
				}
			}),
			$$slots: { default: true }
		});
		$.bind_props($$props, { name });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
