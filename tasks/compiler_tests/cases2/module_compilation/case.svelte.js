let count = $state(0);
const doubled = $derived(count * 2);
export function increment() { count++; }
export function getCount() { return count; }
export function getDoubled() { return doubled; }
$effect(() => { console.log(count); });
