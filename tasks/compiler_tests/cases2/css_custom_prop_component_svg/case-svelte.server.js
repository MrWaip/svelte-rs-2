import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let color = "red";
	$.css_props($$renderer, false, { "--color": color }, () => {
		Child($$renderer, {});
	});
}
