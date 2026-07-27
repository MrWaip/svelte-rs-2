import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let pending = null;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { pending }, ($$anchor) => {
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [() => "awaited"]);
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
