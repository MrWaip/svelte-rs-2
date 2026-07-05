import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	async function process(source) {
		const results = [];
		for await (const item of source) {
			results.push(item);
		}
		return results;
	}
}
