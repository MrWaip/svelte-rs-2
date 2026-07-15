App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<textarea>`);
		$.push_element($$renderer, "textarea", 5, 0);
		const $$body = $.escape(foo);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$$renderer.push(` <textarea>`);
		$.push_element($$renderer, "textarea", 6, 0);
		const $$body_1 = $.escape("hello");
		if ($$body_1) {
			$$renderer.push(`${$$body_1}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$$renderer.push(` <textarea>`);
		$.push_element($$renderer, "textarea", 7, 0);
		const $$body_2 = $.escape(`a${$.stringify(foo)}b`);
		if ($$body_2) {
			$$renderer.push(`${$$body_2}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
