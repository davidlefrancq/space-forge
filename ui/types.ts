// types.ts
export interface Planet {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

export interface SimulationParams {
  date: string; // Date RFC3339
}
