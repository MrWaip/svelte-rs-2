App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<template shadowrootmode="open">`);
		$.push_element($$renderer, "template", 1, 0);
		$$renderer.push(`<slot>`);
		$.push_element($$renderer, "slot", 1, 32);
		$$renderer.push(`</slot>`);
		$.pop_element();
		$$renderer.push(`</template>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
