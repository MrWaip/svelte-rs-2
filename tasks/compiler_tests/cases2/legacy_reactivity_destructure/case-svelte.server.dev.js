App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let point = {
			left: 1,
			right: 2
		};
		let tmp = point, left = tmp.left, right = tmp.right;
		function swap() {
			[left, right] = [right, left];
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`${$.escape(left)}:${$.escape(right)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
