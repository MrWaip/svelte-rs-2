import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function getTag() {
		return "div";
	}
	$.element($$renderer, getTag());
}
