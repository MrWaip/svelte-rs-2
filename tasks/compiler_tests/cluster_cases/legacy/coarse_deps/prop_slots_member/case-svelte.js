import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $.prop($$props, "x", 8);
	Child($$anchor, { prop: $.untrack(() => $$slots.default) });
}
