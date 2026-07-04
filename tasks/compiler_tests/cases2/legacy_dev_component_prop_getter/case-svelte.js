import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	let title = $.prop($$props, "title", 8);
	Child($$anchor, { get title() {
		return title();
	} });
}
