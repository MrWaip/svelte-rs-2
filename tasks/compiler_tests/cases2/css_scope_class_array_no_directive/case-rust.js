import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor) {
	let active = false;
	var div = root();
	$.set_class(div, 1, $.clsx(["container", { active }]), "svelte-wx745y");
	$.append($$anchor, div);
}
