import * as $ from "svelte/internal/client";
const summary = ($$anchor) => {
	var section = root();
	$.append($$anchor, section);
};
var root = $.from_html(`<section class="summary svelte-ic1cb7">summary</section>`);
var root_1 = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	let active = true;
	var div = root_1();
	$.set_class(div, 1, "chunk-shell svelte-ic1cb7", null, {}, { state: active });
	var node = $.child(div);
	summary(node);
	$.reset(div);
	$.append($$anchor, div);
}
