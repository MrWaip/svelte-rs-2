import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let format = $.prop($$props, "format", 3, (x) => x);
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(($0) => $.set_text(text, `${$0 ?? ""}${$$props.extra ?? ""}`), [() => format()($$props.value)]);
	$.append($$anchor, span);
	$.pop();
}
