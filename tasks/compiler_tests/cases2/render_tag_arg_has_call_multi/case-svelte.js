import * as $ from "svelte/internal/client";
const show = ($$anchor, a = $.noop, b = $.noop) => {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""} ${b() ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	{
		let $0 = $.derived(fn1);
		let $1 = $.derived(fn2);
		show($$anchor, () => $.get($0), () => $.get($1));
	}
}
