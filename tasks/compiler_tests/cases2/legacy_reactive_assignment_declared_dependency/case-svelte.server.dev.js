App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let doubled, total;
		let count = 1;
		var step = 2;
		function bump() {
			count += 1;
			step += 1;
		}
		$: doubled = count * 2;
		$: total = doubled + step;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 15, 0);
		$$renderer.push(`${$.escape(doubled)}-${$.escape(total)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
