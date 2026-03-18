import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleScroll() {
		console.log("scrolled");
	}
	$.event("scroll", $.window, handleScroll);
}
