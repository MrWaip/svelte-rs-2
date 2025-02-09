import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div> </div></div> <div><div><div>text</div></div></div> <div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	let name = "world";
	var div = root();
	var div_1 = $.child(div);
	var div_2 = $.child(div_1);
	var text = $.child(div_2, true);
	$.reset(div_2);
	$.reset(div_1);
	var div_3 = $.sibling(div_1, 4);
	var div_4 = $.child(div_3);
	var div_5 = $.child(div_4);
	$.reset(div_4);
	$.reset(div_3);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text, name);
		$.set_attribute(div_5, "name", name);
	});
	$.append($$anchor, div);
}
