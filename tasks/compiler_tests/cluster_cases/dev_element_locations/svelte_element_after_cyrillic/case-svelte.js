import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Текст <!></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.sibling($.child(div));
	$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
		var text = $.text("жирный");
		$.append($$anchor, text);
	});
	$.reset(div);
	$.append($$anchor, div);
}
