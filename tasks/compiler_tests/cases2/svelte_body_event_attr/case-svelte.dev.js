App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleClick() {
		console.log("clicked");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document.body, handleClick);
	return $.pop($$exports);
}
