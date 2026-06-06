import * as $ from "svelte/internal/client";
var optgroup_content = $.from_html(`<em class="hdr"> </em> <option>A</option>`, 1);
var root = $.from_html(`<select><optgroup label="g"><!></optgroup></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let label = $.state("hi");
	var fragment = root();
	var select = $.first_child(fragment);
	var optgroup = $.child(select);
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment_1 = optgroup_content();
		var em = $.first_child(fragment_1);
		var text = $.child(em, true);
		$.reset(em);
		var option = $.sibling(em, 2);
		option.value = option.__value = "a";
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.append(anchor, fragment_1);
	});
	$.reset(select);
	var button = $.sibling(select, 2);
	$.delegated("click", button, () => $.set(label, "bye"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
