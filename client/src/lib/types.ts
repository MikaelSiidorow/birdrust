export interface ParsedReport {
	deviceInformation: DeviceInformation;
	latestCapture: Capture;
	recentViolations: Record<string, ViolationSummary>;
}

export interface DeviceInformation {
	deviceId: string;
	listenRange: number;
	deviceStarted: string;
	uptimeSeconds: number;
	updateIntervalMs: number;
}

export interface Capture {
	snapshotTimestamp: string;
	drones: Drone[];
}

export interface Drone {
	serialNumber: string;
	model: string;
	manufacturer: string;
	mac: string;
	ipv4: string;
	ipv6: string;
	firmware: string;
	positionY: number;
	positionX: number;
	altitude: number;
}

export interface ViolationSummary {
	pilot: Option<Pilot>;
	timesSeen: number;
	closestDistance: number;
	latestDate: string;
}

export interface Pilot {
	pilotId: string;
	firstName: string;
	lastName: string;
	phoneNumber: string;
	email: string;
	createdDate: string;
}

export type Option<T> = T | undefined;
