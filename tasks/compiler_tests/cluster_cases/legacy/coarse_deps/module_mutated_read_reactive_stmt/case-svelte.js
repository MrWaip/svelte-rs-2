import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
let count = 0;
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	function bump() {
		count = count + 1;
	}
	$.legacy_pre_effect(() => {}, () => {
		$.set(doubled, count);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { bump };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	$.bind_prop($$props, "bump", bump);
	return $.pop($$exports);
}
