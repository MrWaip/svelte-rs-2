App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
let count = 0;
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let doubled;
		function bump() {
			count = count + 1;
		}
		$: doubled = count;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`${$.escape(doubled)}</p>`);
		$.pop_element();
		$.bind_props($$props, { bump });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
