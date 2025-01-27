import * as $ from "svelte/internal/client";
var root = $.template(`<br> <br> <br> <div> </div>`, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let name = "";
	$.set(title, 12);
	var fragment = root();
	var text = $.sibling($.first_child(fragment), 1, true);
	var text_1 = $.sibling(text, 2, true);
	var text_2 = $.sibling(text_1, 2);
	var div = $.sibling(text_2);
	var text_3 = $.child(div);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text, name);
		$.set_text(text_1, $.get(title));
		$.set_text(text_2, `_${name ?? ""}_${$.get(title) ?? ""}`);
		$.set_text(text_3, `${name ?? ""}+${$.get(title) ?? ""}`);
	});
	$.append($$anchor, fragment);
}
