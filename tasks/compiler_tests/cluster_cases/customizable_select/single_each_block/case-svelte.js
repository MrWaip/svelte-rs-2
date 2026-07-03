import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div> </div>`);
var select_content = $.from_html(`<!>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor, $$props) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.first_child(fragment);
		$.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
			var div = root_1();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, div);
		});
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
