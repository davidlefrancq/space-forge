'use client';

import { useEffect, useState } from 'react';
import { simulate } from '../lib/api';
import SolarSystem3D from './SolarSystem3D';
import TimeStepSelector from './TimeStepSelector';

const ONE_DAY = 24 * 60 * 60 * 1000

interface CelestItem {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

const SolarSystem = () => {
  const [date, setDate] = useState<Date>(new Date('2025-04-11T00:00:00Z'));
  const [passedTime, setPassedTime] = useState<number>(ONE_DAY);
  const [celestItems, setCelestItems] = useState<CelestItem[]>([]);
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
          const result: CelestItem[] = await simulate(isoDate);
          setCelestItems(result);
          setOrbitHistory((prev) => {
            const newHistory = { ...prev };
            result.forEach((celestItem: CelestItem) => {
              const [x, y, z] = celestItem.position;
              if (!newHistory[celestItem.name]) newHistory[celestItem.name] = [];
              newHistory[celestItem.name].push([x, y, z]);
            });
            return newHistory;
          });
          setDate((d) => new Date(d.getTime() + passedTime));
        } catch (err) {
          console.error(err);
        }
    };

    if (started && !paused) runLoop();

  }, [started, paused, date]);

  return (
    <div className="p-6 text-white space-y-6">
      <div className="flex items-center justify-between bg-zinc-900 p-4 rounded-xl shadow-inner w-full">
        
        {/* Bouton pause à gauche */}
        <div className="flex-1 flex justify-start">
          <button
            onClick={() => setPaused(!paused)}
            className="flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg bg-blue-600 hover:bg-blue-500 transition"
          >
            {paused ? (
              <>
                <span className="text-lg">▶</span> Reprendre
              </>
            ) : (
              <>
                <span className="text-lg">⏸</span> Pause
              </>
            )}
          </button>
        </div>

        {/* Date au centre */}
        <div className="flex-1 flex justify-center font-mono text-sm text-gray-300">
          {String(date.getUTCDate()).padStart(2, '0')}/
          {String(date.getUTCMonth() + 1).padStart(2, '0')}/
          {date.getUTCFullYear()}
        </div>

        {/* Pas temporel à droite */}
        <div className="flex-1 flex justify-end items-center gap-2 text-sm">
          <label className="text-gray-400 whitespace-nowrap">Pas temporel :</label>
          <TimeStepSelector
            initialValue={1}
            initialUnit="day"
            onChange={(s) => setPassedTime(s * 1000)}
          />
        </div>
      </div>
      
      <SolarSystem3D celestItems={celestItems} orbitHistory={orbitHistory} />
    </div>
  );
};

export default SolarSystem;
