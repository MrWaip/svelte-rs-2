import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<a> </a>`);
var root_1 = $.from_html(`<div> <!></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const groups = $.derived(() => {
		const groups = new Map();
		for (const x of $$props.data.schema) groups.set(x, [{
			name: x,
			href: x
		}]);
		return groups;
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $.get(groups), $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let group = () => $.get($$array)[0];
		let links = () => $.get($$array)[1];
		var div = root_1();
		var text = $.child(div);
		var node_1 = $.sibling(text);
		$.each(node_1, 17, links, $.index, ($$anchor, $$item) => {
			let name = () => $.get($$item).name;
			let href = () => $.get($$item).href;
			var a = root_2();
			var text_1 = $.child(a, true);
			$.reset(a);
			$.template_effect(() => {
				$.set_attribute(a, "href", href());
				$.set_text(text_1, name());
			});
			$.append($$anchor, a);
		});
		$.reset(div);
		$.template_effect(() => $.set_text(text, `${group() ?? ""} `));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
}
