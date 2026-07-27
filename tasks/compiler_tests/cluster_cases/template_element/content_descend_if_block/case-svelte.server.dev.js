App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { cond } = $$props;
		$$renderer.push(`<template>`);
		$.push_element($$renderer, "template", 5, 0);
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`a`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></template>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
