App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let onclick = $.fallback($$props["onclick"], undefined);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 4, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { onclick });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
