App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="b"> </div>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<div class="a"></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = [1];
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	$.add_svelte_meta(() => $.each(div, 21, () => items, $.index, ($$anchor, x) => {
		var div_1 = root();
		var text = $.child(div_1, true);
		$.reset(div_1);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append($$anchor, div_1);
	}), "each", App, 6, 1);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
