App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	let $$array = $.tag($.derived(() => $.to_array(items)), "[$derived iterable]"), first = $.tag($.derived(() => $.get($$array)[0]), "first"), second = $.tag($.derived(() => $.get($$array)[1]), "second"), rest = $.tag($.derived(() => $.get($$array).slice(2)), "rest");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(first) ?? ""},${$.get(second) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
