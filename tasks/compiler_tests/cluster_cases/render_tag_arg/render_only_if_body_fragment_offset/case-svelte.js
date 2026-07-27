import * as $ from "svelte/internal/client";
const co = ($$anchor) => {
	var b = root();
	$.append($$anchor, b);
};
var root = $.from_html(`<b>C</b>`);
var option_content = $.from_html(`<span>M</span>`, 1);
var root_1 = $.from_html(`<!> <select><option><!></option></select>`, 1);
export default function App($$anchor) {
	let show = true;
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			co($$anchor);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	var select = $.sibling(node, 2);
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment_2 = option_content();
		$.append(anchor, fragment_2);
	});
	$.reset(select);
	$.append($$anchor, fragment);
}
