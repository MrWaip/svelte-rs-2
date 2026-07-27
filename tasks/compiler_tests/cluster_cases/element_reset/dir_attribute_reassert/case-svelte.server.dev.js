App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 1;
		$$renderer.push(`<p dir="rtl">`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`text</p>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`1<p dir="auto">`);
		$.push_element($$renderer, "p", 6, 8);
		$$renderer.push(`dynamic parent reset</p>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
