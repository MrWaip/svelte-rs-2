import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let p = Promise.resolve({ k1: 1 });
	let num = $.tag($.mutable_source(0), "num");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { [`k${$.update(num)}`]: v } = $.get($$source);
			return { v };
		});
		var v = $.derived_safe_equal(() => $.get($$value).v);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(v) ?? ""} ${$.get(num) ?? ""}`));
		$.append($$anchor, button);
	}), "await", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
