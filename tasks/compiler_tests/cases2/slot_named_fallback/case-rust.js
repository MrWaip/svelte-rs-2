import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>Fallback</p>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "footer", {}, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
