import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const foo = $.effect_tracking();
	let bar = $.state(false);
	$.user_pre_effect(() => {
		$.set(bar, $.effect_tracking(), true);
	});
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo} ${$.get(bar) ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
