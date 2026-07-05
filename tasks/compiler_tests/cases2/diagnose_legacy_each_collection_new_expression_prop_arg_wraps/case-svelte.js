import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let size = $.prop($$props, "size", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(size()), $.untrack(() => new Array(size()).fill(null))), $.index, ($$anchor, _, i) => {
		var div = root();
		div.textContent = i;
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
}
