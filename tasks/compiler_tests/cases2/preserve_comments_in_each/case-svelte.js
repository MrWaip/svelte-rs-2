import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<!-- row --> <span> </span>`, 1);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		var span = $.sibling(node_1, 2);
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
