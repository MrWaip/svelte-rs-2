import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>f</p>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `boundary ${$$props.x ?? ""} text`));
			$.append($$anchor, text);
		});
	}
	$.append($$anchor, fragment);
}
