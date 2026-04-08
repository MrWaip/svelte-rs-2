import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const row = ($$anchor, $$arg0) => {
		let values = $.derived_safe_equal(() => $.fallback($$arg0?.().values, () => [$.get(counter)], true));
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(values).length));
		$.append($$anchor, span);
	};
	let counter = $.state(0);
	$.set(counter, 10);
	row($$anchor, () => ({}));
}
