App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let s = 0;
		let d = $.derived(() => s * 2);
		$$renderer.push(`<input${$.attr("value", d())}/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`s</button>`);
		$.pop_element();
		$$renderer.push(` ${$.escape(d())}`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
