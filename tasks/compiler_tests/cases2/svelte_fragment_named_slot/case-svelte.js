import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<p>First</p> <p>Second</p>`, 1);
export default function App($$anchor) {
	Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var fragment_1 = root();
		$.next(2);
		$.append($$anchor, fragment_1);
	} } });
}
