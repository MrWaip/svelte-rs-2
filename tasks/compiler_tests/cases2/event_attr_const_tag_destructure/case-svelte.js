import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, rest_excludes);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { onClick } = props;
				return { onClick };
			});
			var button = root();
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
