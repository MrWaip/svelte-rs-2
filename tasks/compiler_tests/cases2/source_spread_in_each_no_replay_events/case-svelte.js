import * as $ from "svelte/internal/client";
var root = $.from_html(`<source/>`);
var root_1 = $.from_html(`<picture></picture>`);
export default function App($$anchor, $$props) {
	var picture = root_1();
	$.each(picture, 21, () => $$props.sources, $.index, ($$anchor, source) => {
		var source_1 = root();
		$.attribute_effect(source_1, () => ({ ...$.get(source) }));
		$.append($$anchor, source_1);
	});
	$.reset(picture);
	$.append($$anchor, picture);
}
