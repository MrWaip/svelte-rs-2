import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let localFoo = $.prop($$props, "foo", 3, "default");
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${localFoo() ?? ""} ${$$props.bar ?? ""}`));
	$.append($$anchor, p);
}
