'use client';

import { useState } from 'react';
import { simulate } from '../lib/api';
import PlanetCard from './PlanetCard';
import OrbitCanvas from './OrbitCanvas';

interface Planet {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

const SolarSystem = () => {
  const [date, setDate] = useState('2025-04-11T00:00:00Z');
  const [planets, setPlanets] = useState<Planet[]>([]);
  const [loading, setLoading] = useState(false);

  const runSimulation = async () => {
    setLoading(true);
    try {
      const result = await simulate(date);
      setPlanets(result);
    } catch (error) {
      console.error(error);
    }
    setLoading(false);
  };

  return (
    <div className="p-6 text-white">
      <div className="flex items-center gap-4">
        <input
          type="datetime-local"
          className="bg-gray-700 px-3 py-2 rounded-md"
          value={date.substring(0,16)}
          onChange={(e) => setDate(new Date(e.target.value).toISOString())}
        />
        <button
          className="bg-emerald-600 px-4 py-2 rounded-md hover:bg-emerald-500"
          onClick={runSimulation}
        >
          Simuler
        </button>
      </div>

      {loading && <p className="mt-4">Chargement...</p>}

      <div className="mt-6">
        <OrbitCanvas planets={planets} />
      </div>

      <div className="mt-6 grid grid-cols-2 md:grid-cols-3 gap-4">
        {planets.map((planet) => (
          <PlanetCard key={planet.name} planet={planet} />
        ))}
      </div>
    </div>
  );
};

export default SolarSystem;
