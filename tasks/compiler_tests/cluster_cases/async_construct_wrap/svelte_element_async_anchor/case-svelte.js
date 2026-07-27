import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let deferred = $.proxy(Promise.withResolvers());
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [() => deferred.promise], (node, $$tag) => {
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		});
	});
	$.append($$anchor, fragment);
}
