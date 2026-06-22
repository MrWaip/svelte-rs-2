import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<details><summary>x</summary></details>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let obj = $.prop($$props, "obj", 12);
	$.init();
	var details = root();
	$.bind_property("open", "toggle", details, ($$value) => obj(obj().flag = $$value, true), () => obj().flag);
	$.append($$anchor, details);
	$.pop();
}
