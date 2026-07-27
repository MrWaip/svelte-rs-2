App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let count = 0;
		function bump() {
			count += 1;
		}
		$$renderer.push(`<div${$.attr("title", [() => $$sanitized_props.x])}>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`${$.escape(a)}</div>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`${$.escape(count)}</button>`);
		$.pop_element();
		$.bind_props($$props, { a });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
