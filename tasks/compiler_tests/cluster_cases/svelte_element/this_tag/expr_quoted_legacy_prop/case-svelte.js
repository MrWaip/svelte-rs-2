import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let tag = $.prop($$props, "tag", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, tag, false, ($$element, $$anchor) => {
		var text = $.text("hello");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
