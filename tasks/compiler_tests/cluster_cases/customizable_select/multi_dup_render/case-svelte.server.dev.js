App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { opt } = $$props;
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		opt($$renderer);
		$$renderer.push(`<!----><!></select>`);
		$.pop_element();
		$$renderer.push(` <select>`);
		$.push_element($$renderer, "select", 6, 0);
		opt($$renderer);
		$$renderer.push(`<!----><!></select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
