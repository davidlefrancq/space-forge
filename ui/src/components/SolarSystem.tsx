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
  const [orbitHistory, setOrbitHistory] = useState<Record<string, [number, number][]>>({});
  const [started, setStarted] = useState(false);

  useEffect(() => {
    if (!started) setStarted(true);
  }, []);

  useEffect(() => {  
    runLoop();
  }, [started]);

  useEffect(() => {
    if (started) {
      runLoop();
    }
  }, [orbitHistory]);

  const runLoop = async () => {
    const isoDate = date.toISOString();
    try {
      const result: Planet[] = await simulate(isoDate);
      setPlanets(result);
      setOrbitHistory(prev => {
        const newHistory = { ...prev };
        result.forEach((planet: Planet) => {
          const [x, y] = planet.position;
          if (!newHistory[planet.name]) newHistory[planet.name] = [];
          newHistory[planet.name].push([x, y]);
        });
        return newHistory;
      });
      setDate(d => new Date(d.getTime() + 86400000)); // +1 jour
    } catch (err) {
      console.error(err);
    }

    // Attendre un minimum de temps si tu veux éviter une boucle trop rapide :
    // await new Promise(res => setTimeout(res, 100)); // optionnel
    // runLoop(); // boucle auto-déclenchée
  };  

  return (
    <div className="p-6 text-white">
      {started && <SolarSystem3D planets={planets} />}
    </div>
  );
};

export default SolarSystem;
