import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => "div", false, ($$element, $$anchor) => {
		$.event("click", $$element, function($$arg) {
			$.bubble_event.call(this, $$props, $$arg);
		});
	});
	$.append($$anchor, fragment);
}
