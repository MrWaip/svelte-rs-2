App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>x</span>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let headerTag = $.prop($$props, "headerTag", 3, "div");
	let cond = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(headerTag);
		$.validate_void_dynamic_element(headerTag);
		$.element(node, headerTag, false, ($$element, $$anchor) => {
			var fragment_1 = root_1();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.snippet(node_1, () => $$props.header), "render", App, 7, 1);
			var node_2 = $.sibling(node_1, 2);
			{
				var consequent = ($$anchor) => {
					var span = root();
					$.append($$anchor, span);
				};
				$.add_svelte_meta(() => $.if(node_2, ($$render) => {
					if (cond) $$render(consequent);
				}), "if", App, 8, 1);
			}
			$.append($$anchor, fragment_1);
		}, void 0, [6, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
