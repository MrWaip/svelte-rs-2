App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>Styled</div>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let color = "red";
	let fontSize = "16px";
	let bg = "blue";
	let columns = 3;
	const staticVal = "bold";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		color,
		"--columns": columns,
		"font-size": fontSize,
		"background-color": bg,
		"font-weight": staticVal
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
