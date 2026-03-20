import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var [a, b] = $.get($$source);
			return {
				a,
				b
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var b = $.derived(() => $.get($$value).b);
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} and ${$.get(b) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
