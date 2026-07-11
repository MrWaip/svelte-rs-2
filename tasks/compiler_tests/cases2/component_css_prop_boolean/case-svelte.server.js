import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$.css_props($$renderer, true, { "--foo": true }, () => {
		Child($$renderer, {});
	});
}
