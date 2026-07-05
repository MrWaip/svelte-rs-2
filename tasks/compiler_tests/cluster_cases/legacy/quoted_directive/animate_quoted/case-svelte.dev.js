import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[3, 28]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function flip(node) {
		return {};
	}
	let items = [
		1,
		2,
		3
	];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node_1, 9, () => items, (item) => item, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.animation(div, () => flip, () => ({ duration: 200 }));
		$.append($$anchor, div);
	}), "each", App, 3, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
