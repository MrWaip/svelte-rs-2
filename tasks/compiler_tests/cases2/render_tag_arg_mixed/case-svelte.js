import * as $ from "svelte/internal/client";
const show = ($$anchor, greeting = $.noop, person = $.noop) => {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${greeting() ?? ""} ${person() ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let name = "world";
	function greet() {
		return "hello";
	}
	{
		let $0 = $.derived(greet);
		show($$anchor, () => $.get($0), () => name);
	}
}
