App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!doctype html=""/>`);
		$.push_element($$renderer, "!doctype", 1, 0);
		$.pop_element();
		$$renderer.push(` <html lang="en">`);
		$.push_element($$renderer, "html", 2, 0);
		$$renderer.push(`<head>`);
		$.push_element($$renderer, "head", 3, 1);
		$$renderer.push(`<meta charset="utf-8"/>`);
		$.push_element($$renderer, "meta", 4, 2);
		$.pop_element();
		$$renderer.push(` <title>`);
		$.push_element($$renderer, "title", 5, 2);
		$$renderer.push(`Svelte App</title>`);
		$.pop_element();
		$$renderer.push(`</head>`);
		$.pop_element();
		$$renderer.push(` <body>`);
		$.push_element($$renderer, "body", 7, 1);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 8, 2);
		$$renderer.push(`Hello World</div>`);
		$.pop_element();
		$$renderer.push(`</body>`);
		$.pop_element();
		$$renderer.push(`</html>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
