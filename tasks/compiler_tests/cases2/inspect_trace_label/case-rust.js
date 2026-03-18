import "svelte/internal/flags/tracing";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = $.state(0);
	function handleClick() {
		return $.trace(() => "custom label", () => {
			$.update(count);
		});
	}
}
