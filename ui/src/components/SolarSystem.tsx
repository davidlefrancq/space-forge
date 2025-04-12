'use client';

import { useEffect, useState } from 'react';
import { simulate } from '../lib/api';
import SolarSystem3D from './SolarSystem3D';

interface Planet {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

const SolarSystem = () => {
  const [date, setDate] = useState<Date>(new Date('2025-04-11T00:00:00Z'));
  const [planets, setPlanets] = useState<Planet[]>([]);
  const [orbitHistory, setOrbitHistory] = useState<Record<string, [number, number, number][]>>({});
  const [started, setStarted] = useState(false);
  const [paused, setPaused] = useState(false);

  useEffect(() => {
    if (!started) setStarted(true);
  }, []);

  useEffect(() => {
    const runLoop = async () => {
        const isoDate = date.toISOString();
        try {
          const result: Planet[] = await simulate(isoDate);
          setPlanets(result);
          setOrbitHistory((prev) => {
            const newHistory = { ...prev };
            result.forEach((planet: Planet) => {
              const [x, y, z] = planet.position;
              if (!newHistory[planet.name]) newHistory[planet.name] = [];
              newHistory[planet.name].push([x, y, z]);
            });
            return newHistory;
          });
          const passedTime = 1 * 24 * 60 * 60 * 1000;
          setDate((d) => new Date(d.getTime() + passedTime));
        } catch (err) {
          console.error(err);
        }
    };

    if (started && !paused) runLoop();

  }, [started, paused, date]);

  return (
    <div className="p-6 text-white">
      <div className="mb-4">
        <button
          onClick={() => setPaused(!paused)}
          className="px-4 py-2 bg-blue-600 rounded hover:bg-blue-500 transition"
        >
          {paused ? '▶ Reprendre' : '⏸ Pause'}
        </button>
      </div>
      <SolarSystem3D planets={planets} orbitHistory={orbitHistory} />
    </div>
  );
};

export default SolarSystem;
