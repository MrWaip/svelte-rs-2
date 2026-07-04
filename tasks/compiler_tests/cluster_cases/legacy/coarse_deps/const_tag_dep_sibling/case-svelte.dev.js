import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.prop($$props, "obj", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const name = $.tag($.derived_safe_equal(() => ($.deep_read_state(obj()), $.untrack(() => obj().name))), "name");
			$.get(name);
			const len = $.tag($.derived_safe_equal(() => ($.deep_read_state($.get(name)), $.untrack(() => $.get(name).length))), "len");
			$.get(len);
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""}: ${$.get(len) ?? ""}`));
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (obj()) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
