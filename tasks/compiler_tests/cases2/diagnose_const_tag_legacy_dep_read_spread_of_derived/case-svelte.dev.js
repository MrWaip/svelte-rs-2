import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let cond = $.prop($$props, "cond", 8, false);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const xs = $.tag($.derived_safe_equal(() => [cond() ? 1 : 2]), "xs");
			$.get(xs);
			const ys = $.tag($.derived_safe_equal(() => [3]), "ys");
			$.get(ys);
			const all = $.tag($.derived_safe_equal(() => ($.deep_read_state($.get(xs)), $.deep_read_state($.get(ys)), $.untrack(() => [...$.get(xs), ...$.get(ys)]))), "all");
			$.get(all);
			var text = $.text();
			$.template_effect(() => $.set_text(text, ($.deep_read_state($.get(all)), $.untrack(() => $.get(all).length))));
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (cond()) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
