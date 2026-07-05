App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = [1, 2], $$array = $.tag($.derived(() => $.to_array(tmp, 2)), "[$state iterable]"), x = $.get($$array)[0], y = $.get($$array)[1];
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${x ?? ""} ${y ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
