import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<b> </b><i> </i>`, 1);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $$props.items, $.index, ($$anchor, $$async0) => {
		var fragment_1 = root();
		var b_1 = $.first_child(fragment_1);
		var text = $.child(b_1, true);
		$.reset(b_1);
		var i = $.sibling(b_1);
		var text_1 = $.child(i, true);
		$.reset(i);
		$.template_effect(($0) => {
			$.set_text(text, $.get($$async0));
			$.set_text(text_1, $0);
		}, void 0, [() => $$props.b]);
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
