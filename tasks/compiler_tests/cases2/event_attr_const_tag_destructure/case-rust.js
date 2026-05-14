import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { onClick } = props;
				return { onClick };
			});
			var button = root_1();
			$.delegated("click", button, function(...$$args) {
				$.get(computed_const).onClick?.apply(this, $$args);
			});
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
