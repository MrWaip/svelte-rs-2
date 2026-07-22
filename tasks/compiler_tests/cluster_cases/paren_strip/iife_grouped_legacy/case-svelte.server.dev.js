App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $.fallback($$props["x"], 42);
		let a;
		let b;
		$: b = (function(a) {
			return a;
		})(x);
		$: a = b;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.push(`${$.escape(a)} ${$.escape(b)}</p>`);
		$.pop_element();
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
