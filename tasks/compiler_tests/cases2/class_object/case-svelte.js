import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let isActive = false;
	const bold = true;
	var div = root();
	$.set_class(div, 1, $.clsx({
		active: isActive,
		bold
	}));
	$.append($$anchor, div);
}
