import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const row = ($$anchor) => {
	var p = root();
	$.append($$anchor, p);
};
var root = $.from_html(`<p class="svelte-5iy3wu">hi</p>`);
var root_1 = $.from_html(`<div class="wrap svelte-5iy3wu"><!></div>`);
export default function App($$anchor) {
	var div = root_1();
	var node = $.child(div);
	row(node);
	$.reset(div);
	$.append($$anchor, div);
}
