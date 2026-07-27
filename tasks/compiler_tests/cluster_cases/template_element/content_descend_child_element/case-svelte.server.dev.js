App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x } = $$props;
		$$renderer.push(`<template>`);
		$.push_element($$renderer, "template", 5, 0);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 10);
		$$renderer.push(`${$.escape(x)}</p>`);
		$.pop_element();
		$$renderer.push(`</template>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
