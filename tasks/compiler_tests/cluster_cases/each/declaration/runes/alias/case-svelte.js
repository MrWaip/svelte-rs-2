import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{
		a: 1,
		b: 2
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let x = () => $.get($$item).a;
		let y = () => $.get($$item).b;
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${x() ?? ""}${y() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
