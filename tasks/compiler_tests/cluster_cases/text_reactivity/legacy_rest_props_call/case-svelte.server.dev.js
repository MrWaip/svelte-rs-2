App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["y"]);
	$$renderer.component(($$renderer) => {
		let x = $$props["y"];
		$$renderer.push(`<pre>`);
		$.push_element($$renderer, "pre", 6, 0);
		$$renderer.push(`${$.escape(JSON.stringify($$restProps))}</pre>`);
		$.pop_element();
		$.bind_props($$props, { y: x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
