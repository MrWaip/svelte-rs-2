import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let b = $.prop($$props, "b", 19, () => $$props.a), c = $.prop($$props, "c", 19, () => b() * b());
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$$props.a ?? ""}${b() ?? ""}${c() ?? ""}`));
	$.append($$anchor, p);
}
