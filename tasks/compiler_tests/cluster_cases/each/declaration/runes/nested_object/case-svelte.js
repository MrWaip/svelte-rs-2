import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{
		p: { a: 1 },
		q: { b: 2 }
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).p.a;
		let b = () => $.get($$item).q.b;
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
