import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let action = $.prop($$props, "action", 8);
	let onClick = $.prop($$props, "onClick", 8);
	let onKey = $.prop($$props, "onKey", 8);
	let el = $.mutable_source();
	var div = root();
	$.effect(() => $.event("keydown", div, function(...$$args) {
		onKey()?.apply(this, $$args);
	}));
	$.action(div, ($$node) => action()?.($$node));
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.effect(() => $.event("click", div, function(...$$args) {
		onClick()?.apply(this, $$args);
	}));
	$.append($$anchor, div);
}
