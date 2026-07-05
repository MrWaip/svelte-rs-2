import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { setContext } from "svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let local = 0;
	const getLocal = () => local;
	setContext("k", a());
	local = a();
	var $$exports = {
		...$.legacy_api(),
		get getLocal() {
			return getLocal;
		}
	};
	$.init();
	$.bind_prop($$props, "getLocal", getLocal);
	return $.pop($$exports);
}
