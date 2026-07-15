import * as $ from "svelte/internal/client";
import { derived } from "svelte/store";
import { page } from "$app/stores";
export const route = derived(page, ($page) => {
	if (!$page?.data) {
		return null;
	}
	return $page.url.pathname;
});
