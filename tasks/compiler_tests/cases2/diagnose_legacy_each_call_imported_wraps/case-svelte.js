import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { items } from "./data";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(items), $.untrack(items)), $.index, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
}
