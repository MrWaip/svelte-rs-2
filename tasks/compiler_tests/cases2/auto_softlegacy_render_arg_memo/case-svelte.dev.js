import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { snip } from "./snip.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function getArg() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(getArg);
		$.add_svelte_meta(() => snip($$anchor, () => $.get($0)), "render", App, 8, 0);
	}
	return $.pop($$exports);
}
