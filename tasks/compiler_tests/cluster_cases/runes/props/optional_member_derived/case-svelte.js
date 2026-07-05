import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const props = $.rest_props($$props, rest_excludes);
	let title = $.derived(() => $$props?.title);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(title)));
	$.append($$anchor, text);
	$.pop();
}
