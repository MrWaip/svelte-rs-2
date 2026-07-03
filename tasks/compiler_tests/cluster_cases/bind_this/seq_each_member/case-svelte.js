import * as $ from "svelte/internal/client";
var root = $.from_html(`<b> </b>`);
export default function App($$anchor) {
	let arr = $.proxy([
		1,
		2,
		3
	]);
	let elements = $.proxy([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 18, () => arr, (item) => item, ($$anchor, item, i) => {
		var b = root();
		var text = $.child(b, true);
		$.reset(b);
		$.bind_this(b, (v, i) => elements[i] = v, (i) => elements[i], () => [$.get(i)]);
		$.template_effect(() => $.set_text(text, item));
		$.append($$anchor, b);
	});
	$.append($$anchor, fragment);
}
