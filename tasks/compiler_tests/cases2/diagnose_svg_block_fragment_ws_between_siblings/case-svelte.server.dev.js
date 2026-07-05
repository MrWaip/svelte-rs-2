App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<g>`);
			$.push_element($$renderer, "g", 2, 1);
			$$renderer.push(`<path d="M1">`);
			$.push_element($$renderer, "path", 2, 4);
			$$renderer.push(`</path>`);
			$.pop_element();
			$$renderer.push(`</g>`);
			$.pop_element();
			$$renderer.push(`<g>`);
			$.push_element($$renderer, "g", 3, 1);
			$$renderer.push(`<path d="M2">`);
			$.push_element($$renderer, "path", 3, 4);
			$$renderer.push(`</path>`);
			$.pop_element();
			$$renderer.push(`</g>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
