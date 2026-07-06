App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { val = 0 } = $$props;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 4, 0);
		$$renderer.push(`${$.escape(val)}</p>`);
		$.pop_element();
		$.bind_props($$props, { val });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
