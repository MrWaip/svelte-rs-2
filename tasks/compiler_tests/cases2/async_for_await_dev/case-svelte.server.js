import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function process(source) {
		const results = [];
		for await (const item of source) {
			results.push(item);
		}
		return results;
	}
}
