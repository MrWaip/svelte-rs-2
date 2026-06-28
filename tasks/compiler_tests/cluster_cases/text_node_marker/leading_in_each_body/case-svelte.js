import * as $ from "svelte/internal/client";
var root_1 = $.from_html(` <br/>`, 1);
var root = $.from_html(`<!> <button>add</button>`, 1);
export default function App($$anchor) {
	let array = $.proxy(["A"]);
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 17, () => array, $.index, ($$anchor, a) => {
		$.next();
		var fragment_1 = root_1();
		var text = $.first_child(fragment_1, true);
		$.next();
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, fragment_1);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => array.push("B"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
