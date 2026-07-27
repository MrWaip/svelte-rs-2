import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <div></div>`, 1);
export default function App($$anchor) {
	var color, width;
	var $$promises = $.run([() => Promise.resolve(), () => {
		color = "red";
		width = "1px";
	}]);
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var div_1 = $.sibling(div, 2);
	let styles_1;
	$.template_effect(() => {
		styles = $.set_style(div, "", styles, { color });
		styles_1 = $.set_style(div_1, "", styles_1, { width });
	}, void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, fragment);
}
