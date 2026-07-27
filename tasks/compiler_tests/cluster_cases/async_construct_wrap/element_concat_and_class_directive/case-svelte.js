import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	async function h() {
		return 1;
	}
	async function f() {
		return 2;
	}
	var div = root();
	let classes;
	$.template_effect(($0, $1) => {
		$.set_attribute(div, "title", `z${$0 ?? ""}`);
		classes = $.set_class(div, 1, "", null, classes, $1);
	}, void 0, [() => h(), async () => ({ a: await f() })]);
	$.append($$anchor, div);
}
