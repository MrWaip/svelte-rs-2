import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<li> </li>`);
var root_1 = $.from_html(`<ul></ul>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => items, null, ($$anchor, result) => {
		var ul = root_1();
		$.each(ul, 21, () => $.get(result), $.index, ($$anchor, item) => {
			var li = root_2();
			var text = $.child(li, true);
			$.reset(li);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, li);
		});
		$.reset(ul);
		$.append($$anchor, ul);
	});
	$.append($$anchor, fragment);
}
