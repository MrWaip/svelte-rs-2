import * as $ from "svelte/internal/client";
var root = $.from_html(`<section>Раздел <span>Внутри <em>текст</em></span></section>`);
export default function App($$anchor) {
	var section = root();
	$.append($$anchor, section);
}
