App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
import Icon from "./Icon.svelte";
var root = $.add_locations($.from_html(`<h1>Title</h1> <!> <!>`, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => Button(node, {}), "component", App, 7, 0, { componentTag: "Button" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => Icon(node_1, {}), "component", App, 8, 0, { componentTag: "Icon" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
