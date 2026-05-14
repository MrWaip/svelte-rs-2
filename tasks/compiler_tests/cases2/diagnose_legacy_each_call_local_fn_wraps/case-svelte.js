import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	function items() {
		return [
			1,
			2,
			3
		];
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.untrack(items), $.index, ($$anchor, item) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
