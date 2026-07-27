App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { r } = $$props;
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		r($$renderer);
		$$renderer.push(`<!----><!></select>`);
		$.pop_element();
		$$renderer.push(` <select>`);
		$.push_element($$renderer, "select", 6, 0);
		$$renderer.push(`<optgroup label="g">`);
		$.push_element($$renderer, "optgroup", 6, 8);
		r($$renderer);
		$$renderer.push(`<!----><!></optgroup>`);
		$.pop_element();
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <select>`);
		$.push_element($$renderer, "select", 7, 0);
		$$renderer.option({}, ($$renderer) => {
			$.push_element($$renderer, "option", 7, 8);
			r($$renderer);
			$$renderer.push(`<!---->`);
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
