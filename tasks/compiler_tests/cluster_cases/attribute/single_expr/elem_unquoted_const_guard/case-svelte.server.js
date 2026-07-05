import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<textarea${$.attr("readonly", false, true)}></textarea>`);
}
