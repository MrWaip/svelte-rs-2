import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var data, y;
	var $$promises = $.run([async () => data = await fetch("/a"), () => {
		{
			console.log(1);
			console.log(2);
		}
		y = 1;
	}]);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${data ?? ""}1`), void 0, void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, text);
}
