App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let c = $$props["c"];
		let s = $$props["s"];
		let attributes = $.fallback($$props["attributes"], () => ({}), true);
		$$renderer.push(`<div${$.attributes({
			class: $.clsx(c),
			style: s,
			...attributes
		})}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, {
			c,
			s,
			attributes
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
