App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div title="a b&amp;c&lt;d">x</div> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => Child(node, { label: "a\xA0b&c<d" }), "component", App, 6, 0, { componentTag: "Child" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
