import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	let enabled = true;
	let handler = $.derived(() => enabled ? (node) => {} : null);
	Inner($$renderer, { prop: "value" });
}
