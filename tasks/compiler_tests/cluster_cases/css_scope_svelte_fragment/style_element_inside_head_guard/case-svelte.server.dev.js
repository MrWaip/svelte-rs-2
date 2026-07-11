App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<style>
    body { background: lightblue; }
  </style>`);
		});
		$$renderer.push(`<h1 class="svelte-xildax">`);
		$.push_element($$renderer, "h1", 7, 0);
		$$renderer.push(`hi</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
