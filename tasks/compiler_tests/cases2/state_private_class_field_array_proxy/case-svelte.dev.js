import * as $ from "svelte/internal/client";
export class B {
	#items = $.tag($.state($.proxy([])), "B.#items");
}
