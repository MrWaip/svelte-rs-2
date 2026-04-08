import * as $ from "svelte/internal/client";
export const KIND = "v1";
export function label(name) {
	return `${KIND}:${name}`;
}
const row = ($$anchor, text = $.noop) => {
	var span = root_1();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, text()));
	$.append($$anchor, span);
};
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	{
		let $0 = $.derived(() => $.get(label)($$props.title));
		row($$anchor, () => $.get($0));
	}
}
