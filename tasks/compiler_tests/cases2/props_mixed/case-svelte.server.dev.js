App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b = 10, c = void 0, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`${$.escape(a)}</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.push(`${$.escape(b)}</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape(c)}</p>`);
		$.pop_element();
		$.bind_props($$props, { c });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
