import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	const k = 1;
	Child($$anchor, { prop: k });
}
