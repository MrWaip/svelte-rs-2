import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp)), a = $.proxy($.get($$array)[0]), $$array_1 = $.derived(() => $.to_array($.get($$array).slice(1), 2)), b = $.proxy($.get($$array_1)[0]), c = $.proxy($.get($$array_1)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}`));
	$.append($$anchor, button);
}
