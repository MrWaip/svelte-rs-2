import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	0
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let rest = $.rest_props($$props, rest_excludes);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$$props["0"] ?? ""} ${$$props.foo ?? ""}`));
	$.append($$anchor, text);
	$.pop();
}
