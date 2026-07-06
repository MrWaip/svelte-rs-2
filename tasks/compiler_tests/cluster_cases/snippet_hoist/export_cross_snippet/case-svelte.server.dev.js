App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(one);
function one($$renderer) {
	$.validate_snippet_args($$renderer);
	two($$renderer);
}
$.prevent_snippet_stringification(two);
function two($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<!---->hello`);
}
const message = "hello";
export { one };
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
