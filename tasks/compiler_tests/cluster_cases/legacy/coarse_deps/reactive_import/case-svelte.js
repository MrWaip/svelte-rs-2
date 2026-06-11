import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { x } from "./x.js";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(a()), x), () => {
		a();
		x;
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
