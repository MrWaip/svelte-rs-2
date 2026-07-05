import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<a> </a>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item) => {
		const computed_const = $.tag($.derived_safe_equal(() => {
			const { button } = $.get(item);
			return { button };
		}), "[@const]");
		$.get(computed_const);
		var a = root();
		var text = $.child(a, true);
		$.reset(a);
		$.template_effect(() => {
			$.set_attribute(a, "href", ($.deep_read_state($.get(computed_const).button), $.untrack(() => $.get(computed_const).button.href)));
			$.set_text(text, ($.deep_read_state($.get(computed_const).button), $.untrack(() => $.get(computed_const).button.text)));
		});
		$.append($$anchor, a);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
