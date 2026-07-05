App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<header>`);
		$.push_element($$renderer, "header", 2, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "actions", {}, null);
		$$renderer.push(`<!--]--></header>`);
		$.pop_element();
		$$renderer.push(` <main>`);
		$.push_element($$renderer, "main", 3, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]--></main>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
