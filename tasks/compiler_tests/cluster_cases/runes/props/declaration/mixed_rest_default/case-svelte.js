import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let b = $.prop($$props, "b", 3, 2), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"a",
		"b"
	]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$$props.a ?? ""}${b() ?? ""}${$$props.c ?? ""}`));
	$.append($$anchor, button);
	$.pop();
}
