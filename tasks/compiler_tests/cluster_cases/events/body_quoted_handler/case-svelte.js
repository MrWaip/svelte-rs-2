import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.event("click", $.document.body, () => console.log("x"));
}
