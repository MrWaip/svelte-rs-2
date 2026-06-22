import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = [0];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, x) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, ($.get(x), "")));
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
