App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	const plain = $.tag($.derived(() => (function() {
		return $.get(a);
	})($.get(a))), "plain");
	const named = $.tag($.derived(() => (function f() {
		return $.get(a);
	})($.get(a))), "named");
	const asyncf = $.tag($.derived(() => (async function() {
		return $.get(a);
	})($.get(a))), "asyncf");
	const memb = $.tag($.derived(() => (function() {
		return { x: $.get(a) };
	})($.get(a)).x), "memb");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(plain) ?? ""} ${$.get(named) ?? ""} ${typeof $.get(asyncf)} ${$.get(memb) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(a);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
