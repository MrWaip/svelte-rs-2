import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	function tooltip(node) {
		return { destroy() {} };
	}
	Inner($$renderer, { prop: "value" });
}
