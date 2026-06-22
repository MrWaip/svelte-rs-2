import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root_1 = $.from_html(`<span slot="s">x</span>`);
export default function App($$anchor) {
	A($$anchor, { $$slots: { s: ($$anchor, $$slotProps) => {
		var span = root_1();
		$.append($$anchor, span);
	} } });
}
