import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	let name = $.prop($$props, "name", 8);
	Outer($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var text = $.text();
			$.template_effect(() => $.set_text(text, `hello ${name() ?? ""}`));
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
}
