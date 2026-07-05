import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [[
		1,
		2,
		3
	]];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let length = () => $.get($$item).length;
		let last = () => $.get($$item)[length() - 1];
		let mid = () => $.get($$item)[Math.floor(length() / 2)];
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${last() ?? ""}${mid() ?? ""}${length() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
