App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade, fly } from "svelte/transition";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let show = true;
	let duration = 300;
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
					$.transition(1, $$element, () => fade, () => ({ duration }));
					$.transition(2, $$element, () => fly, () => ({ duration }));
					var text = $.text("x");
					$.append($$anchor, text);
				}, void 0, [9, 1]);
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 8, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
