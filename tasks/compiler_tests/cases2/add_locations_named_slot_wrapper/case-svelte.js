import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<div slot="footer"> </div>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 3, "x");
	Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var div = root();
		var text = $.child(div);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `Footer: ${value() ?? ""}`));
		$.append($$anchor, div);
	} } });
}
