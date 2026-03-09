import * as $ from "svelte/internal/client";
var root = $.template(`<div> </div> <div> </div>`, 1);
export default function App($$anchor) {
	let title = 10;
	let flag = void 0;
	let flag2 = void 0;
	onMount(() => {
		title = 20;
		window.id = title;
		flag2 = title;
		map(title);
	});
	function map(value, off = title) {
		return value;
	}
	const obj = {
		title,
		title
	};
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div);
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	var text_1 = $.child(div_1);
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text, title);
		$.set_attribute(div_1, "flag", flag);
		$.set_text(text_1, flag2);
	});
	$.append($$anchor, fragment);
}
