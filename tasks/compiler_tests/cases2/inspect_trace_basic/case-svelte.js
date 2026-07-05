import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = $.state(0);
	function handleClick() {
		$.update(count);
	}
}
