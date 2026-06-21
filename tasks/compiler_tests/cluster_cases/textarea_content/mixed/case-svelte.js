import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.template_effect(() => $.set_value(textarea, `	<p>not actually an element. ${foo() ?? ""}</p>
`));
	$.append($$anchor, textarea);
}
