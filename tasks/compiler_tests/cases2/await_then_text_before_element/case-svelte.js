import * as $ from "svelte/internal/client";
var root_1 = $.from_html(` <div> </div>`, 1);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, ($$anchor, result) => {
		var fragment_1 = root_1();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(() => {
			$.set_text(text, `text ${$.get(result).name ?? ""} `);
			$.set_text(text_1, $.get(result).value);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
