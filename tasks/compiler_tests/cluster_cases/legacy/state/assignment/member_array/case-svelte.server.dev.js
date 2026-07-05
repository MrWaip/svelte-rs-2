App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let arr = [1, 2];
		function swap() {
			[arr[0], arr[1]] = [arr[1], arr[0]];
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`${$.escape(arr[0])}${$.escape(arr[1])}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
