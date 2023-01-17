import { formatDistanceToNow } from 'date-fns';

export const formatDistance = (distanceMm: number) => (distanceMm / 1000).toFixed(1);
export const formatDate = (date: string) =>
	formatDistanceToNow(Date.parse(date), {
		addSuffix: true
	});
export const formatTimes = (timesSeen: number) => '~' + timesSeen * 2;
