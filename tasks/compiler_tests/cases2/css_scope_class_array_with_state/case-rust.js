import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor, $$props) {
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(["container", $$props.value]), "svelte-wx745y"));
	$.append($$anchor, div);
}
