import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function icon($$renderer) {
	$$renderer.push(`<span>hi</span>`);
}
export default function App($$renderer) {
	Child($$renderer, { icon });
}
