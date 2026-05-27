import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [{
		"a-b": 1,
		"c d": 2
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let ab = () => $.get($$item).ab;
		let cd = () => $.get($$item).cd;
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
