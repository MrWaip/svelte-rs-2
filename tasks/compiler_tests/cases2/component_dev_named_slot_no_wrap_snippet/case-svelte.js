import * as $ from "svelte/internal/client";
import Card from "./Card.svelte";
export default function App($$anchor) {
	Card($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var text = $.text("footer");
		$.append($$anchor, text);
	} } });
}
