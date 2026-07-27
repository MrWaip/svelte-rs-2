import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	let a = $.proxy([]);
	let b = $.proxy([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => a, $.index, ($$anchor, x) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append($$anchor, p);
	}, ($$anchor) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 17, () => b, $.index, ($$anchor, x, $$index, $$array) => {
			var span = root_1();
			var text_1 = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text_1, $.get(x)));
			$.append($$anchor, span);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
