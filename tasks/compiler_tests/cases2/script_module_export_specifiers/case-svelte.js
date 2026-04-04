import * as $ from "svelte/internal/client";
const foo = "foo";
function bar() {
	return foo.toUpperCase();
}
export { foo, bar };
var root = $.from_html(`<p>module exports</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
