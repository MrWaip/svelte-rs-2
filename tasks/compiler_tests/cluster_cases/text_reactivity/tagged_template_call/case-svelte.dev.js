App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function tag(s) {
		return s[0];
	}
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `v ${$0 ?? ""}`), [() => tag``]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
