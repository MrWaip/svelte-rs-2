App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	let tag = "div";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				$.validate_dynamic_element_tag(() => tag);
				$.validate_void_dynamic_element(() => tag);
				$.element(node_1, () => tag, false, ($$element, $$anchor) => {
					var text = $.text("content");
					$.append($$anchor, text);
				}, void 0, [7, 1]);
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
