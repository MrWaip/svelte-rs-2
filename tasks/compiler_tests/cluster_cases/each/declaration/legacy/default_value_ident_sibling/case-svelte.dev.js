import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let array = [{
		a: 1,
		c: 2
	}];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => array, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		a();
		let b = $.derived_safe_equal(() => $.fallback($.get($$item).b, c));
		$.get(b);
		let c = () => $.get($$item).c;
		c();
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(b) ?? ""}${c() ?? ""}`));
		$.append($$anchor, button);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
