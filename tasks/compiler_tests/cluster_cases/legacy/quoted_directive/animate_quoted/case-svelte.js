import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	function flip(node) {
		return {};
	}
	let items = [
		1,
		2,
		3
	];
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.each(node_1, 9, () => items, (item) => item, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.animation(div, () => flip, () => ({ duration: 200 }));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
