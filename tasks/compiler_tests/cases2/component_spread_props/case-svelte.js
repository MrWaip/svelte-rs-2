import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, rest_excludes);
	Button($$anchor, $.spread_props(() => props));
}
