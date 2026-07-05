App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let color = "red";
	let columns = 3;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({
			"--color": color,
			"--columns": columns
		}));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
