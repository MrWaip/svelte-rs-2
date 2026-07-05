App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>x</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let active = false;
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_class(div, 1, $.clsx(["container", { active }]), "svelte-wx745y");
	$.append($$anchor, div);
	return $.pop($$exports);
}
