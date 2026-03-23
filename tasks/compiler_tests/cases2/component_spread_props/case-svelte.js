import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	Button($$anchor, $.spread_props(() => props));
}
