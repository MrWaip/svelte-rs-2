import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	const makeStore = $.derived(() => $$props.config.makeStore);
	const entries = $.derived(() => $.get(makeStore)());
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(entries).x));
	$.append($$anchor, span);
}
