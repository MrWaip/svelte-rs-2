import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.event("click", $.window, () => console.log("x"));
}
