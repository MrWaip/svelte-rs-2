App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<template>`);
		$.push_element($$renderer, "template", 1, 0);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 1, 10);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]--></p>`);
		$.pop_element();
		$$renderer.push(`</template>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
