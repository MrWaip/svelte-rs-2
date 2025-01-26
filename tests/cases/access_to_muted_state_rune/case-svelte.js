import * as $ from "svelte/internal/client";
var root = $.template(`<div> </div><div> </div>`, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let flag = undefined;
	let flag2 = $.state(undefined);
	onMount(() => {
		$.set(title, 20);
		window.id = $.get(title);
		$.set(flag2, $.proxy($.get(title)));
		map($.get(title));
	});
	function map(value, off = $.get(title)) {
		return value;
	}
	const obj = {
		title: $.get(title),
		title: $.get(title)
	};
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	var div_1 = $.sibling(div);
	var text_1 = $.child(div_1, true);
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text, $.get(title));
		$.set_attribute(div_1, "flag", flag);
		$.set_text(text_1, $.get(flag2));
	});
	$.append($$anchor, fragment);
}
