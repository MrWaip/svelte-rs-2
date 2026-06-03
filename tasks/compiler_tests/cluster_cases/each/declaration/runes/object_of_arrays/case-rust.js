import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{
		p: [1, 2],
		q: [3, 4]
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item).p, 2));
		var $$array_1 = $.derived(() => $.to_array($.get($$item).q, 2));
		let a = () => $.get($$array)[0];
		let b = () => $.get($$array)[1];
		let c = () => $.get($$array_1)[0];
		let d = () => $.get($$array_1)[1];
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}${d() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
