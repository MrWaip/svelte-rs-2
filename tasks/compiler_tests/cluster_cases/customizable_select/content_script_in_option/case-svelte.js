import * as $ from "svelte/internal/client";
var option_content = $.with_script($.from_html(`<b>A</b><script>console.log('hi')<\/script>`, 1));
var root = $.from_html(`<select><option><!></option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment = option_content();
		$.next();
		$.append(anchor, fragment);
	});
	option.value = option.__value = "a";
	$.reset(select);
	$.append($$anchor, select);
}
