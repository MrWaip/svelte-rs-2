App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<style>body { background: lightblue; }</style>`), App[$.FILENAME], [[2, 2]]);
var root_1 = $.add_locations($.from_html(`<h1 class="svelte-xildax">hi</h1>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var h1 = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var style = root();
		$.append($$anchor, style);
	});
	$.append($$anchor, h1);
	return $.pop($$exports);
}
