App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_svg(`<g><!></g>`, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let color = "red";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--color": color }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
