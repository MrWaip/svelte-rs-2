import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div slot="activator">hi</div>`);
export default function App($$anchor, $$props) {
	let active = $.prop($$props, "active", 8);
	Outer($$anchor, { $$slots: { activator: ($$anchor, $$slotProps) => {
		var div = root();
		let classes;
		$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, { active: active() }));
		$.append($$anchor, div);
	} } });
}
