import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"a"
]);
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let rest = $.rest_props($$props, rest_excludes);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$$props.a ?? ""}${$$props.b ?? ""}`));
	$.append($$anchor, button);
	$.pop();
}
