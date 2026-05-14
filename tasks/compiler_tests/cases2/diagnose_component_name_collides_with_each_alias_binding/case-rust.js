import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function Modal_1($$anchor) {
	let items = $.proxy([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, Modal) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(Modal)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
