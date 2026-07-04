App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleScroll() {
		console.log("scrolled");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("scroll", $.window, handleScroll);
	return $.pop($$exports);
}
