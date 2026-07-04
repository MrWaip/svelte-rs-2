App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	let items = $.tag($.state([
		1,
		2,
		3
	]), "items");
	let empty = void 0;
	let readonly_obj = { x: 1 };
	$.set(count, 10);
	$.set(count, $.get(count) + 5);
	$.set(items, [
		4,
		5,
		6
	]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, div);
	return $.pop($$exports);
}
