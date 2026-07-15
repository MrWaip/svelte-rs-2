import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>a</div>`);
export default function App($$anchor, $$props) {
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, $.clsx($$props.cls), "svelte-c546ri", classes, { active: $$props.on }));
	$.append($$anchor, div);
}
