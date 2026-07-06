App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = $.fallback($$props["title"], undefined);
		$$renderer.push(`<div${$.attr("title", title)}>`);
		$.push_element($$renderer, "div", 4, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { title });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
