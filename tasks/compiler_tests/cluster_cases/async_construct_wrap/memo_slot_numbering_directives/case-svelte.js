import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor) {
	function fn() {
		return "red";
	}
	var div = root();
	let classes;
	let styles;
	$.template_effect(($0, $1) => {
		classes = $.set_class(div, 1, "", null, classes, $1);
		styles = $.set_style(div, "", styles, $0);
	}, [() => ({ color: fn() })], [async () => ({ a: await true })]);
	$.append($$anchor, div);
}
