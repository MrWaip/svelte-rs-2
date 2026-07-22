import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = $.state(0);
	const plain = $.derived(() => (function() {
		return $.get(a);
	})($.get(a)));
	const named = $.derived(() => (function f() {
		return $.get(a);
	})($.get(a)));
	const asyncf = $.derived(() => (async function() {
		return $.get(a);
	})($.get(a)));
	const memb = $.derived(() => (function() {
		return { x: $.get(a) };
	})($.get(a)).x);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(plain) ?? ""} ${$.get(named) ?? ""} ${typeof $.get(asyncf)} ${$.get(memb) ?? ""}`));
	$.delegated("click", button, () => $.update(a));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
