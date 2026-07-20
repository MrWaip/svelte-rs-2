import * as $ from "svelte/internal/client";
var optgroup_content = $.with_script($.from_html(`<option>A</option> <script>console.log('hi')<\/script>`, 1));
var root = $.from_html(`<select><optgroup label="g"><!></optgroup></select>`);
export default function App($$anchor) {
	var select = root();
	var optgroup = $.child(select);
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment = optgroup_content();
		var option = $.first_child(fragment);
		option.value = option.__value = "a";
		$.next(2);
		$.append(anchor, fragment);
	});
	$.reset(select);
	$.append($$anchor, select);
}
