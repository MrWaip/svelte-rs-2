import * as $ from "svelte/internal/client";
var root = $.template(`<input>`);
export default function App($$anchor) {
	let title = $.state(10);
	let name = "";
	$.set(title, 12);
	var input = root();
	$.template_effect(() => {
		$.set_attribute(input, "name", name);
		$.set_attribute(input, "name2", name);
		$.set_attribute(input, "title", $.get(title));
		$.set_attribute(input, "title2", $.get(title));
		$.set_attribute(input, "complex", `_${$.get(title) ?? ""}+${name ?? ""}_`);
	});
	$.append($$anchor, input);
}
