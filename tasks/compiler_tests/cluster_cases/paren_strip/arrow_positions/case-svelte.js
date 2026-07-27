import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = $.state(0);
	const handler = $.derived(() => $.get(a) ? () => {
		$.update(a);
	} : undefined);
	const list = $.derived(() => [() => $.get(a), () => $.get(a) + 1]);
	const cfg = $.derived(() => ({ run: () => $.get(a) }));
	const call = $.derived(() => [1].map(() => $.get(a)));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${$.get(a) ?? ""} ${$.get(list).length ?? ""} ${$0 ?? ""} ${$.get(call).length ?? ""}`), [() => $.get(cfg).run()]);
	$.delegated("click", button, function(...$$args) {
		$.get(handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
