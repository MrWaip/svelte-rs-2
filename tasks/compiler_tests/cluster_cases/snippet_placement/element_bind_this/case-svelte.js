import * as $ from "svelte/internal/client";
var root = $.from_html(`<form></form> `, 1);
export default function App($$anchor) {
	let thisBug = $.state(void 0);
	var fragment = root();
	var form = $.first_child(fragment);
	{
		const Bug = ($$anchor) => {
			$.next();
			var text = $.text("cool");
			$.append($$anchor, text);
		};
		$.bind_this(form, ($$value) => $.set(thisBug, $$value), () => $.get(thisBug));
	}
	var text_1 = $.sibling(form);
	$.template_effect(() => $.set_text(text_1, ` ${typeof $.get(thisBug)}`));
	$.append($$anchor, fragment);
}
