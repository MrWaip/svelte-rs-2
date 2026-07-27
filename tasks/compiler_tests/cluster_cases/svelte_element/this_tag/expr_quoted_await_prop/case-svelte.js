import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [() => $$props.pending], (node, $$tag) => {
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		});
	});
	$.append($$anchor, fragment);
}
