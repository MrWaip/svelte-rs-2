App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function handleClick() {
			count++;
		}
		function getHandler() {
			return handleClick;
		}
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 13, 0);
		$$renderer.push(`${$.escape(count)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
