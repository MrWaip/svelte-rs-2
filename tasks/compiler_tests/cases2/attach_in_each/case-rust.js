import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let items = $.proxy([
		"a",
		"b",
		"c"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.attach(div, () => tooltip);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
