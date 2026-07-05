import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var foo = $.prop($$props, "foo", 8, 1);
	function getFoo() {
		return foo();
	}
	var $$exports = {
		...$.legacy_api(),
		get getFoo() {
			return getFoo;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, p);
	$.bind_prop($$props, "getFoo", getFoo);
	return $.pop($$exports);
}
