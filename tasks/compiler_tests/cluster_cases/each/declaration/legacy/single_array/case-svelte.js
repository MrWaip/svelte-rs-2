import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [[1, 2]];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let a = () => $.get($$array)[0];
		let b = () => $.get($$array)[1];
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
