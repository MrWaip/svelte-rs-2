App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 22]]);
var root_1 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 22]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.each(node_1, 17, () => $$props.items, $.index, ($$anchor, item) => {
				var span = root();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, $.get(item)));
				$.append($$anchor, span);
			}), "each", App, 6, 1);
			$.append($$anchor, fragment_1);
		};
		var alternate = ($$anchor) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.add_svelte_meta(() => $.each(node_2, 17, () => $$props.items, $.index, ($$anchor, item) => {
				var div = root_1();
				var text_1 = $.child(div, true);
				$.reset(div);
				$.template_effect(() => $.set_text(text_1, $.get(item)));
				$.append($$anchor, div);
			}), "each", App, 8, 1);
			$.append($$anchor, fragment_2);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.loading) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
