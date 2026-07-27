import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	async function g() {
		return 1;
	}
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...{ q: 1 },
		[$.STYLE]: $0
	}), void 0, [async () => ({ color: `${await g() ?? ""}px` })]);
	$.append($$anchor, div);
}
