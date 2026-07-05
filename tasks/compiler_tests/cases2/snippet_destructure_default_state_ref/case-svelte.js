import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const row = ($$anchor, $$arg0) => {
		let values = $.derived_safe_equal(() => $.fallback(($$arg0?.()).values, () => [counter], true));
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(values).length));
		$.append($$anchor, span);
	};
	let counter = 0;
	row($$anchor, () => ({}));
}
