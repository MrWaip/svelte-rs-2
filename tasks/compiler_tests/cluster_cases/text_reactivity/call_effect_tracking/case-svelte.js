import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `v ${$0 ?? ""}`), [() => $.effect_tracking()]);
	$.append($$anchor, p);
}
