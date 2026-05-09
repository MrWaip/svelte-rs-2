import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	function fmt(x) {
		return x;
	}
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(($0) => $.set_text(text, `${$0 ?? ""} • ${$$props.card.tail ?? ""}`), [() => $$props.card.flag ? "A" : "B" + fmt($$props.card.x)]);
	$.append($$anchor, span);
	$.pop();
}
