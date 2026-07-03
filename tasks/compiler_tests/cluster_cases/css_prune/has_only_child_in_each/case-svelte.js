import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div class="b svelte-10ib7zp"> </div>`);
var root = $.from_html(`<div class="a svelte-10ib7zp"></div>`);
export default function App($$anchor) {
	let items = [1];
	var div = root();
	$.each(div, 21, () => items, $.index, ($$anchor, x) => {
		var div_1 = root_1();
		var text = $.child(div_1, true);
		$.reset(div_1);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append($$anchor, div_1);
	});
	$.reset(div);
	$.append($$anchor, div);
}
