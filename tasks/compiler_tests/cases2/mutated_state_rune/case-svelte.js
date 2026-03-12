import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div> <div> </div>`, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let flag = void 0;
	let flag2 = $.state(void 0);
	let value = $.state("text");
	onMount(() => {
		$.set(title, 20);
		window.id = $.get(title);
		$.set(flag2, $.get(title), true);
		map($.get(title));
	});
	function map(value, off = $.get(title)) {
		return value;
	}
	$.set(value, $.get(value) + 1234);
	$.set(value, $.get(value) - 4e3);
	$.set(value, $.get(value) * 2);
	$.set(value, $.get(value) && fallback, true);
	$.set(value, "");
	const obj = {
		title: $.get(title),
		title: $.get(title)
	};
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	$.set_attribute(div_1, "flag", flag);
	var text_1 = $.child(div_1, true);
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text, $.get(title));
		$.set_text(text_1, $.get(flag2));
	});
	$.append($$anchor, fragment);
}
