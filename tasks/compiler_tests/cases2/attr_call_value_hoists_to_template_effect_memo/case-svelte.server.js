import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function getTitle() {
		return "hello";
	}
	$$renderer.push(`<div${$.attr("title", getTitle())}></div>`);
}
