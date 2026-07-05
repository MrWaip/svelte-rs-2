import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var select_content = $.from_html(`<!>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.first_child(fragment);
		Inner(node, {});
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
