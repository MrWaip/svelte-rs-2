import * as $ from "svelte/internal/client";
var optgroup_content = $.from_html(`<!>`, 1);
var root = $.from_html(`<optgroup><!></optgroup>`);
export default function App($$anchor, $$props) {
	var optgroup = root();
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment = optgroup_content();
		var node = $.first_child(fragment);
		$.snippet(node, () => $$props.children);
		$.append(anchor, fragment);
	});
	$.append($$anchor, optgroup);
}
