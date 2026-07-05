App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		function getTotal() {
			const total = $.derived(() => {
				let sum = 0;
				for (const item of items) {
					sum += item;
				}
				return sum;
			});
			return total();
		}
		$.bind_props($$props, { getTotal });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
