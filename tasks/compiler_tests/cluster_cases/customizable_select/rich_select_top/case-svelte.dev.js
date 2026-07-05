App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<span class="hdr"> </span><option>A</option>`, 1), App[$.FILENAME], [[6, 1], [7, 1]]);
var root = $.add_locations($.from_html(`<select><!></select> <button>x</button>`, 1), App[$.FILENAME], [[5, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let label = $.tag($.state("hi"), "label");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var select = $.first_child(fragment);
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment_1 = select_content();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		var option = $.sibling(span);
		option.value = option.__value = "a";
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.append(anchor, fragment_1);
	});
	var button = $.sibling(select, 2);
	$.delegated("click", button, function click() {
		return $.set(label, "bye");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
