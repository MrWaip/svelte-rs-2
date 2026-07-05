App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["bar"]);
	$$renderer.component(($$renderer) => {
		function foo() {}
		$$renderer.push(`<div${$.attributes({ ...$$restProps })}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`${$.escape(foo())}</div>`);
		$.pop_element();
		$.bind_props($$props, { bar: foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
