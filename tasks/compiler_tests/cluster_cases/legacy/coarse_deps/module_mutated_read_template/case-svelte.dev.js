import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
let count = 0;
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function bump() {
		count = count + 1;
	}
	var $$exports = {
		...$.legacy_api(),
		get bump() {
			return bump;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count));
	$.append($$anchor, p);
	$.bind_prop($$props, "bump", bump);
	return $.pop($$exports);
}
