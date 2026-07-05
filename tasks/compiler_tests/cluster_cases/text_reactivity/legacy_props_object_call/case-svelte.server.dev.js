App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let y = $$props["y"];
		$$renderer.push(`<pre>`);
		$.push_element($$renderer, "pre", 5, 0);
		$$renderer.push(`${$.escape(JSON.stringify($$sanitized_props))}</pre>`);
		$.pop_element();
		$.bind_props($$props, { y });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
