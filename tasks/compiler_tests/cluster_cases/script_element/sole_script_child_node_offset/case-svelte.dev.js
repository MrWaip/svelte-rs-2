App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.with_script($.from_html(`<div><script><\/script><!></div> <!>`, 1)), App[$.FILENAME], [[
	1,
	0,
	[[1, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node_1 = $.sibling($.first_child(fragment), 2);
	$.html(node_1, () => x);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
