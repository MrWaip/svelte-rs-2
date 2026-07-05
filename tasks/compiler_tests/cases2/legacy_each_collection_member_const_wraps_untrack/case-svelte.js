import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const meta = { items: [
		1,
		2,
		3
	] };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.untrack(() => meta.items), $.index, ($$anchor, item) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
