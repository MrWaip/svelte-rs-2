import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host"
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let props = $.rest_props($$props, rest_excludes);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $$props.x));
	$.append($$anchor, text);
	$.pop();
}
$.create_custom_element(App, {}, [], [], { mode: "open" });
