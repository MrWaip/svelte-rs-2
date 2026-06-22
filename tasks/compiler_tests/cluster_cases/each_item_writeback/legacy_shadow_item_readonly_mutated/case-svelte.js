import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let a = $.mutable_source(["Hello"]);
	function go() {
		$.set(a, [...$.get(a), "x"]);
	}
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(a), $.index, ($$anchor, a, $$index, $$array) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, text);
	});
	var button = $.sibling(node, 2);
	$.event("click", button, go);
	$.append($$anchor, fragment);
}
