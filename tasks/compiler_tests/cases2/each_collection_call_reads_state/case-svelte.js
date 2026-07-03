import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span></span>`);
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 8, 3);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(count()), $.untrack(() => Array.from({ length: count() }))), $.index, ($$anchor, _) => {
		var span = root();
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
