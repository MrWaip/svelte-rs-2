import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let n = 0;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => (() => {
		$.effect_root(() => {
			n;
		});
		return n;
	})()]);
	$.append($$anchor, p);
	$.pop();
}
