import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	let local = undefined;
	const getLocal = () => local;
	local = a() + b();
	var $$exports = {
		...$.legacy_api(),
		get getLocal() {
			return getLocal;
		}
	};
	$.bind_prop($$props, "getLocal", getLocal);
	return $.pop($$exports);
}
