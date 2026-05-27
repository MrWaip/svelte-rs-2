import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([[
		1,
		2,
		3
	]]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let a = () => $.get($$array)[0];
		let c = () => $.get($$array)[1];
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${c() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
