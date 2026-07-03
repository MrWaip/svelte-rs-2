import * as $ from "svelte/internal/client";
const t = ($$anchor) => {
	var p = root();
	$.append($$anchor, p);
};
var root = $.from_html(`<p>hi</p>`);
export default function App($$anchor) {
	t($$anchor);
}
