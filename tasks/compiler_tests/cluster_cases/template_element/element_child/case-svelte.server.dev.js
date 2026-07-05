App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<template id="t1">`);
		$.push_element($$renderer, "template", 1, 0);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 2, 1);
		$$renderer.push(`foo</div>`);
		$.pop_element();
		$$renderer.push(`</template>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
