import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = [{}];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let w = $.derived_safe_equal(() => $.fallback($.get($$item).w, 1));
		$.get(w);
		let x = $.derived_safe_equal(() => $.fallback($.get($$item).x, $.get(w) * 2));
		$.get(x);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(w) ?? ""}${$.get(x) ?? ""}`));
		$.append($$anchor, button);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
