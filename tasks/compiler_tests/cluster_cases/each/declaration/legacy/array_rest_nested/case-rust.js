import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [[
		1,
		2,
		3
	]];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item)));
		var $$array_1 = $.derived(() => $.to_array($.get($$array).slice(1), 2));
		let a = () => $.get($$array)[0];
		let b = () => $.get($$array_1)[0];
		let c = () => $.get($$array_1)[1];
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
