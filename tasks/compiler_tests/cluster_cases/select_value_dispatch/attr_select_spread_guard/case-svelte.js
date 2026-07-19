import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>Dog</option></select>`);
export default function App($$anchor, $$props) {
	var select = root();
	$.attribute_effect(select, () => ({
		value: "dog",
		...$$props.props
	}));
	var option = $.child(select);
	option.value = option.__value = "dog";
	$.reset(select);
	$.append($$anchor, select);
}
