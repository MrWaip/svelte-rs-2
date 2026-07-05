App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let test = $.fallback($$props["test"], () => [], true);
		let hidden = false;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`${$.escape(hidden ? "show" : "hide")} b</button>`);
		$.pop_element();
		$$renderer.push(` <label>`);
		$.push_element($$renderer, "label", 10, 0);
		$$renderer.push(`a <input type="checkbox"${$.attr("checked", test.includes("a"), true)} value="a"/>`);
		$.push_element($$renderer, "input", 10, 9);
		$.pop_element();
		$$renderer.push(`</label>`);
		$.pop_element();
		$$renderer.push(` `);
		if (!hidden) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<label>`);
			$.push_element($$renderer, "label", 12, 1);
			$$renderer.push(`b <input type="checkbox"${$.attr("checked", test.includes("b"), true)} value="b"/>`);
			$.push_element($$renderer, "input", 12, 10);
			$.pop_element();
			$$renderer.push(`</label>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <label>`);
		$.push_element($$renderer, "label", 14, 0);
		$$renderer.push(`c${$.escape(hidden ? "show" : "hide")} b <input value="just here, so b is not the last input"/>`);
		$.push_element($$renderer, "input", 14, 37);
		$.pop_element();
		$$renderer.push(`</label>`);
		$.pop_element();
		$.bind_props($$props, { test });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
