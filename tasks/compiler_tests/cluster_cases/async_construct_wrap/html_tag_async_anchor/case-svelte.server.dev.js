import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let deferred = Promise.withResolvers();
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 5, 0);
		$$renderer.child_block(async ($$renderer) => {
			$$renderer.push($.html((await $.save(deferred.promise))()));
		});
		$$renderer.push(`</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
