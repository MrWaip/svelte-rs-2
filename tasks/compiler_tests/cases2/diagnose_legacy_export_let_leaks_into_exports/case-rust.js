import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	let local = undefined;
	const getLocal = () => local;
	local = a() + b();
	var $$exports = { getLocal };
	$.bind_prop($$props, "getLocal", getLocal);
	return $.pop($$exports);
}
