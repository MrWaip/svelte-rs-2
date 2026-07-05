App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[9, 4]]);
var root_1 = $.add_locations($.from_html(`<div>before <!> after</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.set(count, 1);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.sibling($.child(div));
	$.add_svelte_meta(() => $.key(node, () => $.get(count) % 2, ($$anchor) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(count)));
		$.append($$anchor, span);
	}), "key", App, 8, 2);
	$.next();
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
