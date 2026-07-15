App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { format = (x) => x, value = void 0, extra } = $$props;
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`${$.escape(format(value))}${$.escape(extra)}</span>`);
		$.pop_element();
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
