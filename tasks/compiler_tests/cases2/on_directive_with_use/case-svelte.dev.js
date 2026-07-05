import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let action = $.prop($$props, "action", 8);
	let onClick = $.prop($$props, "onClick", 8);
	let el = $.tag($.mutable_source(), "el");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	$.slot(node, $$props, "default", {}, null);
	$.reset(div);
	$.action(div, ($$node) => action()?.($$node));
	$.effect(() => $.event("keydown", div, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	}));
	$.effect(() => $.event("click", div, function(...$$args) {
		$.apply(onClick, this, $$args, App, [9, 37]);
	}));
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, div);
	return $.pop($$exports);
}
