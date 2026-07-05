import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let visible = $.prop($$props, "visible", 8, false);
	let items = $.prop($$props, "items", 24, () => [{
		value: "a",
		ref: null
	}]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.each(node_1, 1, items, $.index, ($$anchor, item, $$index) => {
				var div = root();
				var text = $.child(div, true);
				$.reset(div);
				$.bind_this(div, ($$value, item) => (item.ref = $$value, $.invalidate_inner_signals(() => items())), (item) => item?.ref, () => [$.get(item)]);
				$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).value))));
				$.append($$anchor, div);
			}), "each", App, 7, 1);
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (visible()) $$render(consequent);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
