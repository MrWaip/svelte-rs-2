App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.prop($$props, "count", 7, 0);
	var $$exports = {
		...$.legacy_api(),
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
