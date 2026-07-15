import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $$d = $.derived(() => $$props.confirmStore.data), phone = $.derived(() => $.get($$d).phone), rate = $.derived(() => $.get($$d).rate);
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `${$.get(phone) ?? ""}${$.get(rate) ?? ""}`));
	$.append($$anchor, span);
	$.pop();
}
