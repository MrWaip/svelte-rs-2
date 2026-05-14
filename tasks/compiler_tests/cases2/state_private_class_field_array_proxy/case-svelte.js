import * as $ from "svelte/internal/client";
export class B {
	#items = $.state($.proxy([]));
}
