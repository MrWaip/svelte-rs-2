App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let p = $.tag_proxy($.proxy(Promise.resolve({
		a: 1,
		b: 2,
		c: 3
	})), "p");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { a, ...rest } = $.get($$source);
			return {
				a,
				rest
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var rest = $.derived(() => $.get($$value).rest);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest).b ?? ""}`));
		$.append($$anchor, button);
	}), "await", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
