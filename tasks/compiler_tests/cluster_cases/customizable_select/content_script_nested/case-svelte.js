import * as $ from "svelte/internal/client";
var select_content = $.with_script($.from_html(`<option>A</option><div><script>console.log('hi')<\/script><!></div>`, 1));
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var option = $.first_child(fragment);
		option.value = option.__value = "a";
		$.next();
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
