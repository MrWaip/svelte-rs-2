App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	$.add_svelte_meta(() => Button(node, {}), "component", App, 6, 1, { componentTag: "Button" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
