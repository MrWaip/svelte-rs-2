App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.prop($$props, "items", 19, () => []), config = $.prop($$props, "config", 19, getDefault), label = $.prop($$props, "label", 3, "hello");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
