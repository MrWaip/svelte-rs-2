import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let count = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.key(node, () => count, ($$anchor) => {
		const doubled = $.derived(() => count * 2);
		var p = root_1();
		p.textContent = $.get(doubled);
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
