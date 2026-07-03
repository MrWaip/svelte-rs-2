import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([[{ a: 1 }, { b: 2 }]]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let a = () => $.get($$array)[0].a;
		let b = () => $.get($$array)[1].b;
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
