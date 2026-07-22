App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = false;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => "p");
		$.validate_void_dynamic_element(() => "p");
		$.element(node, () => "p", false, ($$element, $$anchor) => {
			var text = $.text("before");
			$.append($$anchor, text);
		}, void 0, [5, 0]);
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_2 = $.first_child(fragment_1);
			{
				$.validate_dynamic_element_tag(() => "strong");
				$.validate_void_dynamic_element(() => "strong");
				$.element(node_2, () => "strong", false, ($$element_1, $$anchor) => {
					var text_1 = $.text("during");
					$.append($$anchor, text_1);
				}, void 0, [7, 1]);
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 6, 0);
	}
	var node_3 = $.sibling(node_1, 2);
	{
		$.validate_dynamic_element_tag(() => "p");
		$.validate_void_dynamic_element(() => "p");
		$.element(node_3, () => "p", false, ($$element_2, $$anchor) => {
			var text_2 = $.text("after");
			$.append($$anchor, text_2);
		}, void 0, [9, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
