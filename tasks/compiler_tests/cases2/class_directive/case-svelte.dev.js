App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>Lorem</div>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let absolute = $.tag($.state(void 0), "absolute");
	let visible = $.tag($.state(void 0), "visible");
	let unchanged = void 0;
	let untouched = void 0;
	const staticClass = true;
	$.set(visible, 12);
	$.set(absolute, true);
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
