App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x = 0 } = $$props;
		function fmt(n) {
			return String(n);
		}
		$$renderer.push(`<div${$.attr_class("", void 0, { "active": x === 0 })}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 6, 2);
		$$renderer.push(`${$.escape(fmt(x))}</span>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
