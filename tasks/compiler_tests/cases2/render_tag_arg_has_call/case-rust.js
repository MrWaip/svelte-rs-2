import * as $ from "svelte/internal/client";
const show = ($$anchor, data = $.noop) => {
	var p = root_1();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, data()));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	{
		let $0 = $.derived(getData);
		show($$anchor, () => $.get($0));
	}
}
