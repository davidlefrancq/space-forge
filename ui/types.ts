// types.ts
export interface CelestItem {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

export interface SimulatorParams {
  date: string; // Date RFC3339
}
