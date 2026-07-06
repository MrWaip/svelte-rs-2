App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = 10;
		let title2 = 12;
		title--;
		++title2;
		$$renderer.push(`<div${$.attr("attr", title++)}>`);
		$.push_element($$renderer, "div", 8, 0);
		$$renderer.push(`_</div>`);
		$.pop_element();
		$$renderer.push(` ${$.escape(--title2)}`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
