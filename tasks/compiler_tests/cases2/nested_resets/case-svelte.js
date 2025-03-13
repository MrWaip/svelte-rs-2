import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div></div></div> <div><div><div>text</div></div></div> <div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	var div = root();
	var div_1 = $.child(div);
	var div_2 = $.child(div_1);
	div_2.textContent = name;
	$.reset(div_1);
	var div_3 = $.sibling(div_1, 4);
	var div_4 = $.child(div_3);
	var div_5 = $.child(div_4);
	$.set_attribute(div_5, "name", name);
	$.reset(div_4);
	$.reset(div_3);
	$.reset(div);
	$.append($$anchor, div);
}
