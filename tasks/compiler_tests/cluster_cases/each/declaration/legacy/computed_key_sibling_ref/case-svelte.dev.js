import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = [[
		1,
		2,
		3
	]];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let length = () => $.get($$item).length;
		length();
		let last = () => $.get($$item)[length() - 1];
		last();
		let mid = () => $.get($$item)[Math.floor(length() / 2)];
		mid();
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${last() ?? ""}${mid() ?? ""}${length() ?? ""}`));
		$.append($$anchor, button);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
