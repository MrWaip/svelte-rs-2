App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import tooltip from "./tooltip.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleClick() {
		console.log("clicked");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document.body, handleClick);
	$.action($.document.body, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => "hello");
	return $.pop($$exports);
}
