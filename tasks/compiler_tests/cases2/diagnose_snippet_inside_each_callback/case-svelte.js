import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		const row = ($$anchor, v = $.noop) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, v()));
			$.append($$anchor, p);
		};
		row($$anchor, () => $.get(item));
	});
	$.append($$anchor, fragment);
}
