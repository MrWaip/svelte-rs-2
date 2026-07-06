App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["checked"]);
	$$renderer.component(($$renderer) => {
		let checked = $.fallback($$props["checked"], false);
		function k() {}
		$$renderer.push(`<div${$.attributes({ ...$$restProps })}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`<input${$.attr("checked", checked, true)} type="checkbox"/>`);
		$.push_element($$renderer, "input", 8, 4);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { checked });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
