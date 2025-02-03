import * as $ from "svelte/internal/client";
var root = $.template(`<div>Lorem</div>`);
export default function App($$anchor) {
	let absolute = $.state(undefined);
	let visible = $.state(undefined);
	let unchanged = undefined;
	let untouched = undefined;
	$.set(visible, 12);
	$.set(absolute, true);
	var div = root();
	$.template_effect(() => {
		$.toggle_class(div, "visible", $.get(visible));
		$.toggle_class(div, "absolute", $.get(absolute));
		$.toggle_class(div, "unchanged", unchanged);
		$.toggle_class(div, "untouched", untouched);
	});
	$.append($$anchor, div);
}
