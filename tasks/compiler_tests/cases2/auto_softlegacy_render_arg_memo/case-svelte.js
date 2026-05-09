import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { snip } from "./snip.js";
export default function App($$anchor) {
	function getArg() {
		return 1;
	}
	{
		let $0 = $.derived_safe_equal(getArg);
		snip($$anchor, () => $.get($0));
	}
}
