import * as $ from "svelte/internal/client";
var root_1 = $.from_html(` <div> </div>`, 1);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, items, $.index, ($$anchor, item) => {
		$.next();
		var fragment_1 = root_1();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div);
		$.reset(div);
		$.template_effect(() => {
			$.set_text(text, `${$.get(item) ?? ""} `);
			$.set_text(text_1, `${$.get(item) ?? ""} + example`);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
