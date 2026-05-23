import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.from_html(`<div class="island"><!>  <!></div>`);
export default function App($$anchor) {
	var div = root();
	var node = $.child(div);
	A(node, {});
	var node_1 = $.sibling(node, 2);
	B(node_1, {});
	$.reset(div);
	$.append($$anchor, div);
}
