import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.from_html(`<div>text</div>`);
export default function App($$anchor) {
	let items = $.proxy([
		"a",
		"b",
		"c"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var div = root();
		$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => $.get(item));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
