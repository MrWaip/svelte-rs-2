import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"a"
]);
var root = $.from_html(`<p>foo exists</p>`);
var root_1 = $.from_html(`<p> </p> <!>`, 1);
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let rest = $.rest_props($$props, rest_excludes);
	var fragment = root_1();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var node = $.sibling(p, 2);
	{
		var consequent = ($$anchor) => {
			var p_1 = root();
			$.append($$anchor, p_1);
		};
		$.if(node, ($$render) => {
			if ($$slots.foo) $$render(consequent);
		});
	}
	$.template_effect(($0) => $.set_text(text, `${$$props.a ?? ""} ${$0 ?? ""}`), [() => Object.keys(rest)]);
	$.append($$anchor, fragment);
}
