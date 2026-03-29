import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let count = $.state(0);
	function handleClick() {
		$.update(count);
	}
	function getHandler() {
		return handleClick;
	}
	var div = root();
	var event_handler = $.derived(getHandler);
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", div, handleClick);
	$.event("scroll", div, handleClick);
	$.event("click", div, handleClick, true);
	$.event("focus", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
$.delegate(["click"]);
