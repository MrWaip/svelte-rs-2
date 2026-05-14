import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const obj = { list: [
		1,
		2,
		3
	] };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => obj.list, $.index, ($$anchor, item) => {
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
