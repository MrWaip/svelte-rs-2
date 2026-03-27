import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let tag = $.prop($$props, "tag", 3, "p");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, tag, false, ($$element, $$anchor) => {
		var text = $.text();
		$.template_effect(() => $.set_text(text, `Hello ${$$props.name ?? ""}!`));
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
