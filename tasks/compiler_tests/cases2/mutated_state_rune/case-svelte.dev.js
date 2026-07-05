App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div> <div> </div>`, 1), App[$.FILENAME], [[27, 0], [29, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = $.tag($.state(10), "title");
	let flag = void 0;
	let flag2 = $.tag($.state(void 0), "flag2");
	let value = $.tag($.state("text"), "value");
	onMount(() => {
		$.set(title, 20);
		window.id = $.get(title);
		$.set(flag2, $.get(title), true);
		map($.get(title));
	});
	function map(value, off = $.get(title)) {
		return value;
	}
	$.set(value, $.get(value) + 1234);
	$.set(value, $.get(value) - 4e3);
	$.set(value, $.get(value) * 2);
	$.set(value, $.get(value) && fallback, true);
	$.set(value, "");
	const obj = {
		title: $.get(title),
		title: $.get(title)
	};
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	$.set_attribute(div_1, "flag", flag);
	var text_1 = $.child(div_1, true);
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text, $.get(title));
		$.set_text(text_1, $.get(flag2));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
