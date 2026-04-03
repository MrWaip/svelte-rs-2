import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let url = "/api";
	function outer() {
		async function inner() {
			let $$d = (await $.save($.async_derived(() => fetch(url).then((r) => r.json()))))(), data = $.derived(() => $.get($$d).data), meta = $.derived(() => $.get($$d).meta);
			return 1;
		}
		return inner;
	}
	var $$exports = { outer };
	return $.pop($$exports);
}
