App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div title="a b&amp;c&lt;d">`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`x</div>`);
		$.pop_element();
		$$renderer.push(` `);
		Child($$renderer, { label: "a\xA0b&c<d" });
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
