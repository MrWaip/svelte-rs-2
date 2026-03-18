import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { name, age } = $.get($$source);
			return {
				name,
				age
			};
		});
		var name = $.derived(() => $.get($$value).name);
		var age = $.derived(() => $.get($$value).age);
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} is ${$.get(age) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
