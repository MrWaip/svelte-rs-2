App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "" } = $$props;
		let group = [];
		$$renderer.push(`<input type="radio"${$.attr("checked", group === "a", true)} value="a"/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", group === "b", true)} value="b"/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.push(`${$.escape(value)}</p>`);
		$.pop_element();
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
