App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if (true) {
			$$renderer.push("<!--[0-->");
			const bar = "world";
			const foo = bar;
			const yoo = foo;
			$$renderer.push(`<h1>`);
			$.push_element($$renderer, "h1", 6, 1);
			$$renderer.push(`Hello worldworld!</h1>`);
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
