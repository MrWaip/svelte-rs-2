App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const foo = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	$.next();
	var text = $.text("oo");
	$.append($$anchor, text);
});
export { foo };
var root = $.add_locations($.from_html(`<h1></h1>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	h1.textContent = "Hello world!";
	$.append($$anchor, h1);
	return $.pop($$exports);
}
