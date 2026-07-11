App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.derived(() => 0), "count");
	let postfix = $.update(count);
	let postfix_minus = $.update(count, -1);
	let prefix = $.update_pre(count);
	let prefix_minus = $.update_pre(count, -1);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${postfix ?? ""}, ${postfix_minus ?? ""}, ${prefix ?? ""}, ${prefix_minus ?? ""}, ${$.get(count) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
