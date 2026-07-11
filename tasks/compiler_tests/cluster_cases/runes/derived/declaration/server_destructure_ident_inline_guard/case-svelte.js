import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	const phone = $.derived(() => $$props.source.phone), rate = $.derived(() => $$props.source.rate);
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `${$.get(phone) ?? ""}${$.get(rate) ?? ""}`));
	$.append($$anchor, span);
}
