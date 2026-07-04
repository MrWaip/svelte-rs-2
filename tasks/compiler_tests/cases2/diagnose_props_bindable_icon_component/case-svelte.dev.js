App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="ui-input-icon svelte-nbptzh"><!></div>`), App[$.FILENAME], [[22, 2]]);
var root_1 = $.add_locations($.from_html(`<div class="ui-input-wrapper svelte-nbptzh"><!> <input class="ui-input svelte-nbptzh"/></div>`), App[$.FILENAME], [[
	20,
	0,
	[[26, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.prop($$props, "value", 11, ""), placeholder = $.prop($$props, "placeholder", 3, ""), type = $.prop($$props, "type", 3, "text"), disabled = $.prop($$props, "disabled", 3, false);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var div_1 = root();
			var node_1 = $.child(div_1);
			$.add_svelte_meta(() => $.component(node_1, () => $$props.icon, ($$anchor, Icon_1) => {
				Icon_1($$anchor, {});
			}), "component", App, 23, 3, { componentTag: "Icon" });
			$.reset(div_1);
			$.append($$anchor, div_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.icon) $$render(consequent);
		}), "if", App, 21, 1);
	}
	var input = $.sibling(node, 2);
	$.remove_input_defaults(input);
	$.reset(div);
	$.template_effect(() => {
		$.set_value(input, value());
		$.set_attribute(input, "placeholder", placeholder());
		$.set_attribute(input, "type", type());
		input.disabled = disabled();
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
