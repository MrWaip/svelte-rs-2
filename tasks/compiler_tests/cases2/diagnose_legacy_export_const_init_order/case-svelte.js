import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { setContext } from "svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	let local = 0;
	const getLocal = () => local;
	setContext("k", a());
	local = a();
	var $$exports = { getLocal };
	$.init();
	$.bind_prop($$props, "getLocal", getLocal);
	return $.pop($$exports);
}
