import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	let action = $.prop($$props, "action", 8);
	let onClick = $.prop($$props, "onClick", 8);
	let el = $.mutable_source();
	var div = root();
	var node = $.child(div);
	$.slot(node, $$props, "default", {}, null);
	$.reset(div);
	$.action(div, ($$node) => action()?.($$node));
	$.effect(() => $.event("keydown", div, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	}));
	$.effect(() => $.event("click", div, function(...$$args) {
		onClick()?.apply(this, $$args);
	}));
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, div);
}
