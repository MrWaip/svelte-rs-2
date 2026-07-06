App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(foo);
function foo($$renderer, a, b) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<!---->Hello world ${$.escape(a + b)}`);
}
export { foo };
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
