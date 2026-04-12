import * as $ from "svelte/internal/client";
const pair = ($$anchor) => {
	var div = root_1();
	$.append($$anchor, div);
};
var root_1 = $.from_html(`<div class="after svelte-1hn6tgg">after</div>`);
var root = $.from_html(`<span class="before svelte-1hn6tgg">before</span> <!> <div>other</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	pair(node);
	$.next(2);
	$.append($$anchor, fragment);
}
