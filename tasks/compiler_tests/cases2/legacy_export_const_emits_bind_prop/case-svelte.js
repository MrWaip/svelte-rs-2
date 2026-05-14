import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const greet = () => "hi";
	var $$exports = { greet };
	$.bind_prop($$props, "greet", greet);
	return $.pop($$exports);
}
