import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let tmp = {
		x: "a",
		z: ["b"]
	}, $$array = $.derived(() => $.to_array(tmp.z, 1)), foo = $.prop($$props, "foo", 24, () => $.fallback(tmp.x, "default-x")), bar = $.prop($$props, "bar", 24, () => $.get($$array)[0]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo() ?? ""}${bar() ?? ""}`));
	$.append($$anchor, p);
}
