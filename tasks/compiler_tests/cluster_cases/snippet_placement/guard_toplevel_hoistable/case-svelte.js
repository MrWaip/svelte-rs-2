import * as $ from "svelte/internal/client";
const t = ($$anchor) => {
	var p = root_1();
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p>hi</p>`);
export default function App($$anchor) {
	t($$anchor);
}
