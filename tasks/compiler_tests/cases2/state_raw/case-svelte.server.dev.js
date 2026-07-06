App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let items = [
			1,
			2,
			3
		];
		let empty = void 0;
		let readonly_obj = { x: 1 };
		count = 10;
		count += 5;
		items = [
			4,
			5,
			6
		];
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 12, 0);
		$$renderer.push(`${$.escape(count)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
