import "svelte/internal/flags/tracing";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = $.state(0);
	const handleClick = () => {
		return $.trace(() => "handleClick (3:21)", () => {
			$.update(count);
		});
	};
}
