import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => [
		1,
		2,
		3
	], $.index, ($$anchor, item) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, item));
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
