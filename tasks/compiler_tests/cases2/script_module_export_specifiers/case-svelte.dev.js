App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const foo = "foo";
function bar() {
	return foo.toUpperCase();
}
export { foo, bar };
var root = $.add_locations($.from_html(`<p>module exports</p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.append($$anchor, p);
	return $.pop($$exports);
}
