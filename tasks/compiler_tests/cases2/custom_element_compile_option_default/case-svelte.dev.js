App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.prop($$props, "count", 7, 0);
	var $$exports = {
		...$.legacy_api(),
		get count() {
			return count();
		},
		set count($$value = 0) {
			count($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
$.create_custom_element(App, { count: {} }, [], [], { mode: "open" });
