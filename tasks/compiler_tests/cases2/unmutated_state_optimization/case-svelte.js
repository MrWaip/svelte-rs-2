import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<li> </li>`);
var root = $.from_html(`<button> </button> <p></p> <ul></ul>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	let label = "hello";
	let items = $.proxy([
		1,
		2,
		3
	]);
	function increment() {
		$.set(count, $.get(count) + 1);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var p = $.sibling(button, 2);
	p.textContent = "hello";
	var ul = $.sibling(p, 2);
	$.each(ul, 21, () => items, $.index, ($$anchor, item) => {
		var li = root_1();
		var text_1 = $.child(li, true);
		$.reset(li);
		$.template_effect(() => $.set_text(text_1, $.get(item)));
		$.append($$anchor, li);
	});
	$.reset(ul);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, increment);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
