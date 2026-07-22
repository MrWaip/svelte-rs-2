import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = $.state(0);
	const g1 = $.derived(() => $.get(a) && (() => $.get(a)));
	const g2 = $.derived(() => $.get(a) ?? (() => $.get(a)));
	const g3 = $.derived(() => () => ({ x: $.get(a) }));
	const g4 = $.derived(() => ($.get(a), () => $.get(a)));
	const g5 = $.derived(() => ($.get(a) + 1) * 2);
	const g6 = $.derived(() => (() => $.get(a))());
	const g7 = $.derived(() => (function() {
		return $.get(a);
	})());
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0, $1) => $.set_text(text, `${$.get(g5) ?? ""} ${$.get(g6) ?? ""} ${$.get(g7) ?? ""} ${typeof $.get(g1)} ${typeof $.get(g2)} ${$0 ?? ""} ${$1 ?? ""}`), [() => $.get(g3)().x, () => $.get(g4)()]);
	$.delegated("click", button, () => $.update(a));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
