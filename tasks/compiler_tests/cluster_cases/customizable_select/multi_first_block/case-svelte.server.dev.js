App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { header, footer } = $$props;
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		header($$renderer);
		$$renderer.push(`<!----><div>`);
		$.push_element($$renderer, "div", 5, 26);
		$$renderer.push(`x</div>`);
		$.pop_element();
		footer($$renderer);
		$$renderer.push(`<!----><!></select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
