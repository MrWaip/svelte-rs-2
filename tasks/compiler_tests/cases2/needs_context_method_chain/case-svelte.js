import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let items = $.proxy([]);
	let total = $.derived(() => {
		return items.filter(Boolean).length;
	});
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(total)));
	$.append($$anchor, p);
	$.pop();
}
