import { readable } from 'svelte/store';
import type { ParsedReport } from '$lib/types';
import { PUBLIC_WS_URL } from '$env/static/public';

export const report = readable<ParsedReport>(undefined, (set) => {
	const ws = new WebSocket(PUBLIC_WS_URL);
	ws.onmessage = (event) => {
		const data: ParsedReport = JSON.parse(event.data);
		set(data);
	};
	return () => ws.close();
});
