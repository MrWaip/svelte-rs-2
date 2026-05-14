import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let item = $.prop($$props, "item", 8);
	$.init();
	var div = root();
	$.set_style(div, "", {}, { display: "flex" });
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(item()), $.untrack(() => item().text))));
	$.append($$anchor, div);
	$.pop();
}
