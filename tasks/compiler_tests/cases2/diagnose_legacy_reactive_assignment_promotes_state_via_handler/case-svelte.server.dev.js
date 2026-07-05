App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let trigger = $$props["trigger"];
		let value;
		function read() {
			return value;
		}
		$: value = trigger;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 15, 0);
		$$renderer.push(`</button>`);
		$.pop_element();
		$.bind_props($$props, { trigger });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
