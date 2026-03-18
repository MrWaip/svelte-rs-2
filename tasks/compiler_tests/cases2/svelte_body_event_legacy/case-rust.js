import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleClick() {
		console.log("clicked");
	}
	$.event("click", $.document.body, handleClick);
}
