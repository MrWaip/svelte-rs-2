import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [{
		a: 1,
		b: 2
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let x = () => $.get($$item).a;
		let y = () => $.get($$item).b;
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${x() ?? ""}${y() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
