import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => $$props.keys, (key) => key, ($$anchor, key) => {
		const column = $.derived(() => $$props.columns[key]);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(column)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
}
