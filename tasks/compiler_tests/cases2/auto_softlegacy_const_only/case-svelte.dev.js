import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 18]]);
var root_1 = $.add_locations($.from_html(`<p></p> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const greeting = "hello";
	const items = [
		1,
		2,
		3
	];
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var p = $.first_child(fragment);
	p.textContent = "hello";
	var node = $.sibling(p, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, i) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(i)));
		$.append($$anchor, span);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
