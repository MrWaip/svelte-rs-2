App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.add_locations($.from_html(`<div class="island"><!>  <!></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	$.add_svelte_meta(() => A(node, {}), "component", App, 7, 4, { componentTag: "A" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => B(node_1, {}), "component", App, 10, 4, { componentTag: "B" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
