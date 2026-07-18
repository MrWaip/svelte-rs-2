import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <span> </span>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.each(node, 16, () => [1, 2], $.index, ($$anchor, n, $$index, $$array) => {
		const value = n * 10;
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, value));
		$.append($$anchor, p);
	});
	var span = $.sibling(node, 2);
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, $$props.value));
	$.append($$anchor, fragment);
}
