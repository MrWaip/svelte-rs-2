import * as $ from "svelte/internal/client";
import tooltip from "./tooltip.js";
export default function App($$anchor) {
	function handleClick() {
		console.log("clicked");
	}
	$.event("click", $.document.body, handleClick);
	$.action($.document.body, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => "hello");
}
