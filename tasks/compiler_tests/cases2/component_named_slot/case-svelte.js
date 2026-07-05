import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<p slot="footer">Footer</p>`);
export default function App($$anchor) {
	Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } });
}
