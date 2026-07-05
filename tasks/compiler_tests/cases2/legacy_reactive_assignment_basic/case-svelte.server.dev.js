App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let sum;
		let label = $.fallback($$props["label"], "sum");
		let a = 1;
		let b = 2;
		$: sum = a + b;
		$: console.log(`${label}: ${sum}`);
		$: ((param) => {
			via_iife = param * 2;
		})(sum);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 14, 0);
		$$renderer.push(`${$.escape(sum)}-${$.escape(via_iife)}</p>`);
		$.pop_element();
		$.bind_props($$props, { label });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
