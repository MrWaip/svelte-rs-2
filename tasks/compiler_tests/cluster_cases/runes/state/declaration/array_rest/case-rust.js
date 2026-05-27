import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp)), a = $.proxy($.get($$array)[0]), rest = $.proxy($.get($$array).slice(1));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${rest.length ?? ""}`));
	$.append($$anchor, button);
}
