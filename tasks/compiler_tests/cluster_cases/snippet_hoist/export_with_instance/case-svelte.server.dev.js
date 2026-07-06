App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(foo);
function foo($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<!---->oo`);
}
export { foo };
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "world";
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 9, 0);
		$$renderer.push(`Hello world!</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
