App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let html = "";
		let text = "";
		let content = "";
		$$renderer.push(`<div contenteditable="">`);
		$.push_element($$renderer, "div", 7, 0);
		if (html) {
			$$renderer.push(`${html}`);
		} else {}
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <div contenteditable="">`);
		$.push_element($$renderer, "div", 9, 0);
		const $$body = $.escape(text);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <div contenteditable="">`);
		$.push_element($$renderer, "div", 11, 0);
		const $$body_1 = $.escape(content);
		if ($$body_1) {
			$$renderer.push(`${$$body_1}`);
		} else {}
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
