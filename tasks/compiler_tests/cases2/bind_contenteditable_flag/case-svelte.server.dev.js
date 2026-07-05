App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let html = "";
		$$renderer.push(`<div contenteditable="true">`);
		$.push_element($$renderer, "div", 5, 0);
		if (html) {
			$$renderer.push(`${html}`);
		} else {
			$$renderer.push(`text ${$.escape(html)}`);
		}
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
