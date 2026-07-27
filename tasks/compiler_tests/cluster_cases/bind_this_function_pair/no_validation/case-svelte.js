import * as $ from "svelte/internal/client";
import Test from "./Test.svelte";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let entries = $.proxy([]);
	var fragment = root();
	var node = $.first_child(fragment);
	$.bind_this(Test(node, {}), ($$value) => entries[0] = $$value, () => entries?.[0]);
	var node_1 = $.sibling(node, 2);
	$.bind_this(Test(node_1, {}), (v) => entries[1] = v, () => entries[1]);
	$.append($$anchor, fragment);
}
