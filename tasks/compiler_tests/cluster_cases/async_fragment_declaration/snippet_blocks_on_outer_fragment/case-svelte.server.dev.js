import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		let number;
		$.prevent_snippet_stringification(row);
		function row($$renderer) {
			$.validate_snippet_args($$renderer);
			let doubled;
			var promises_1 = $$renderer.run([() => promises[0], () => doubled = number() * 2]);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 9, 1);
			$$renderer.async([promises_1[1]], ($$renderer) => $$renderer.push(() => $.escape(doubled)));
			$$renderer.push(`</span>`);
			$.pop_element();
		}
		var promises = $$renderer.run([async () => number = await $.async_derived(async () => (await $.save(Promise.resolve(n)))())]);
		row($$renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
