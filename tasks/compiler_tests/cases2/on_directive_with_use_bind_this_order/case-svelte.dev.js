import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let action = $.prop($$props, "action", 8);
	let onClick = $.prop($$props, "onClick", 8);
	let onKey = $.prop($$props, "onKey", 8);
	let el = $.tag($.mutable_source(), "el");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.effect(() => $.event("keydown", div, function(...$$args) {
		$.apply(onKey, this, $$args, App, [10, 17]);
	}));
	$.action(div, ($$node) => action()?.($$node));
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.effect(() => $.event("click", div, function(...$$args) {
		$.apply(onClick, this, $$args, App, [10, 60]);
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
