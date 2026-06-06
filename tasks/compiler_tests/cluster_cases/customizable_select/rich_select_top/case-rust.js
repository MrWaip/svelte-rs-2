import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<span class="hdr"> </span><option>A</option>`, 1);
var root = $.from_html(`<select><!></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let label = $.state("hi");
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
	$.delegated("click", button, () => $.set(label, "bye"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
