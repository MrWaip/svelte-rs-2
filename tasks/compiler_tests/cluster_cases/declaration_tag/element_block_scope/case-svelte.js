import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	const active = $$props.cls === "on";
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_class(div, 1, $.clsx(active ? "a" : "b"));
		$.set_text(text, active);
	});
	$.append($$anchor, div);
}
