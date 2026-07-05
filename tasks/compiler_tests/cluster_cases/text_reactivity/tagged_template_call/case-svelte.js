import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	function tag(s) {
		return s[0];
	}
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `v ${$0 ?? ""}`), [() => tag``]);
	$.append($$anchor, p);
}
