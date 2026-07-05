App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function handleClick() {
		$.update(count);
	}
	function getHandler() {
		return handleClick;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var event_handler = $.derived(getHandler);
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", div, handleClick);
	$.event("scroll", div, handleClick);
	$.event("click", div, handleClick, true);
	$.event("focus", div, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [17, 13], true, true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
$.delegate(["click"]);
