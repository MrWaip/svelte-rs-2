import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let getText = $.prop($$props, "getText", 3, (item) => String(item));
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(($0) => $.set_text(text, `${$0 ?? ""}${$$props.getUrl ?? ""}`), [() => getText()(1)]);
	$.append($$anchor, span);
	$.pop();
}
