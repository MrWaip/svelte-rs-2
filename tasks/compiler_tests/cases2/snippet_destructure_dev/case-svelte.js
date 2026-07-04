import * as $ from "svelte/internal/client";
const greeting = ($$anchor, $$arg0) => {
	let label = () => ($$arg0?.()).label;
	let name = $.derived_safe_equal(() => $.fallback(($$arg0?.()).name, "world"));
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${label() ?? ""}: ${$.get(name) ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	greeting($$anchor, () => ({ label: "Hi" }));
}
