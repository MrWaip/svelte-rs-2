App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
export const meta = { title: "x" };
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let one = 1;
		let doubled = 0;
		$: doubled = one * 2;
		$$renderer.push(`<input type="radio"${$.attr("checked", one === 1, true)}${$.attr("value", 1)}/>`);
		$.push_element($$renderer, "input", 11, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", one === 2, true)}${$.attr("value", 2)}/>`);
		$.push_element($$renderer, "input", 12, 0);
		$.pop_element();
		$$renderer.push(` <span>`);
		$.push_element($$renderer, "span", 13, 0);
		$$renderer.push(`${$.escape(doubled)}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
