import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[15, 22]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	let inserted = false;
	function shouldShow() {
		if (inserted) {
			return false;
		}
		inserted = true;
		return true;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var p = root();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(item)));
				$.append($$anchor, p);
			};
			var d = $.derived(() => $.untrack(shouldShow));
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(d)) $$render(consequent);
			}), "if", App, 15, 4);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 14, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
