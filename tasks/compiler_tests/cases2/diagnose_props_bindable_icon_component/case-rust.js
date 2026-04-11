import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div class="ui-input-icon svelte-nbptzh"><!></div>`);
var root = $.from_html(`<div class="ui-input-wrapper svelte-nbptzh"><!> <input class="ui-input svelte-nbptzh"/></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 11, ""), placeholder = $.prop($$props, "placeholder", 3, ""), type = $.prop($$props, "type", 3, "text"), disabled = $.prop($$props, "disabled", 3, false);
	var div = root();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var div_1 = root_1();
			var node_1 = $.child(div_1);
			$.component(node_1, () => $$props.icon, ($$anchor, Icon_1) => {
				Icon_1($$anchor, {});
			});
			$.reset(div_1);
			$.append($$anchor, div_1);
		};
		$.if(node, ($$render) => {
			if ($$props.icon) $$render(consequent);
		});
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
	$.pop();
}
