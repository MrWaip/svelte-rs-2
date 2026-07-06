App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(pair);
function pair($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div class="after svelte-1hn6tgg">`);
	$.push_element($$renderer, "div", 8, 4);
	$$renderer.push(`after</div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<span class="before svelte-1hn6tgg">`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`before</span>`);
		$.pop_element();
		$$renderer.push(` `);
		pair($$renderer);
		$$renderer.push(`<!----> <div>`);
		$.push_element($$renderer, "div", 13, 0);
		$$renderer.push(`other</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
