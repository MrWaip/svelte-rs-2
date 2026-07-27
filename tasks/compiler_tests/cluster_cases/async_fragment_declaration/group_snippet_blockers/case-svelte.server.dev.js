import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if (true) {
			$$renderer.push("<!--[0-->");
			let number;
			$.prevent_snippet_stringification(greet);
			function greet($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<h1>`);
				$.push_element($$renderer, "h1", 4, 2);
				$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(number)));
				$$renderer.push(`</h1>`);
				$.pop_element();
			}
			var promises = $$renderer.run([async () => number = (await $.save(Promise.resolve(5)))()]);
			greet($$renderer);
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
