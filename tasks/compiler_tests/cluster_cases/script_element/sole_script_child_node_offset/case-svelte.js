import * as $ from "svelte/internal/client";
var root = $.with_script($.from_html(`<div><script><\/script><!></div> <!>`, 1));
export default function App($$anchor) {
	var fragment = root();
	var node_1 = $.sibling($.first_child(fragment), 2);
	$.html(node_1, () => x);
	$.append($$anchor, fragment);
}
