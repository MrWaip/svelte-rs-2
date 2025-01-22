import * as $ from "svelte/internal/client";
var root = $.template(`<div> </div><div></div>`, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let flag = undefined;
	onMount(() => {
		title = 20;
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
	$.template_effect(() => {
		$.set_text(text, title);
		$.set_attribute(div_1, "flag", flag);
	});
	$.append($$anchor, fragment);
}
