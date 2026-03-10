import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Lorem</div>`);
export default function App($$anchor) {
	let absolute = $.state(void 0);
	let visible = $.state(void 0);
	let unchanged = void 0;
	let untouched = void 0;
	const staticClass = true;
	$.set(visible, 12);
	$.set(absolute, true);
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, {
		visible: $.get(visible),
		absolute: $.get(absolute),
		unchanged,
		untouched,
		staticClass,
		static2: true
	}));
	$.append($$anchor, div);
}
