import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><p class="before svelte-105a842">before</p> <!> <p class="foo svelte-105a842"><span class="svelte-105a842">foo</span></p> <p class="bar svelte-105a842">bar</p></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.sibling($.child(div), 2);
	$.slot(node, $$props, "default", {}, null);
	$.next(4);
	$.reset(div);
	$.append($$anchor, div);
}
