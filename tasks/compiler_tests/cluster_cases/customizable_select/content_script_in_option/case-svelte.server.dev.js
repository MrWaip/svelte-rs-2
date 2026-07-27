App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 1, 0);
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 2, 1);
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 2, 19);
			$$renderer.push(`A</b>`);
			$.pop_element();
			$$renderer.push(`<script>console.log('hi')<\/script>`);
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
