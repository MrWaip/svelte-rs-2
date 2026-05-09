import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	Widget($$anchor, { $$events: {
		focus($$arg) {
			$.bubble_event.call(this, $$props, $$arg);
		},
		keydown($$arg) {
			$.bubble_event.call(this, $$props, $$arg);
		}
	} });
}
