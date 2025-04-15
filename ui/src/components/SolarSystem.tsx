'use client';

import { useEffect, useState } from 'react';
import { getSimulatedRange, simulate } from '../lib/api';
import SolarSystem3D from './SolarSystem3D';
import TimeStepSelector from './TimeStepSelector';

const ONE_DAY = 24 * 60 * 60 * 1000

interface CelestItem {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
  timestamp: string;
}

const SolarSystem = () => {
  const [date, setDate] = useState<Date>(new Date('2025-04-11T00:00:00Z'));
  const [passedTime, setPassedTime] = useState<number>(ONE_DAY);
  const [celestItems, setCelestItems] = useState<CelestItem[]>([]);
  const [orbitHistory, setOrbitHistory] = useState<Record<string, [number, number, number][]>>({});
  const [started, setStarted] = useState(false);
  const [paused, setPaused] = useState(true);
  const [loadeded, setLoaded] = useState(false);

  useEffect(() => {
    if (!started) setStarted(true);
  }, []);

  useEffect(() => {
    if (started && !paused) run();

  }, [started, paused, date]);

  const run = async () => {
    const getDataRange = async (start: Date, end: Date) => {
      const result: CelestItem[] = await getSimulatedRange(
        start.toISOString(),
        end.toISOString(),
        passedTime / 1000
      );
      console.log(result);
      return result;
    }
    try {
      if (!loadeded) {
        const startDate = new Date(date.getTime());
        const endDate = new Date(date.getTime() + passedTime * 10);
        const result = await getDataRange(startDate, endDate);
        dataProcessing(result);
      } else {
        const newDate = new Date(date.getTime() + passedTime);
        const result: CelestItem[] = await simulate(newDate.toISOString());
        dataProcessing(result);
      }
    } catch (err) {
      console.error(err);
    }
  };

  const dataProcessing = async (result: CelestItem[]) => {
    if (result && result.length > 0) {
      // Update celest items
      setCelestItems((prev) => {
        const newCelestItems = [...prev];
        result.forEach((celestItem: CelestItem) => {
          const existingItem = newCelestItems.find(item => item.name === celestItem.name);
          if (existingItem) {
            existingItem.position = celestItem.position;
            existingItem.velocity = celestItem.velocity;
          } else {
            newCelestItems.push(celestItem);
          }
        });
        return newCelestItems;
      })
      
      // Update orbit history
      setOrbitHistory((prev) => {
        const newHistory = { ...prev };
        result.forEach((celestItem: CelestItem) => {
          const [x, y, z] = celestItem.position;
          if (!newHistory[celestItem.name]) newHistory[celestItem.name] = [];
          const existingPosition = newHistory[celestItem.name].find(pos => pos[0] === x && pos[1] === y && pos[2] === z);
          if (!existingPosition) newHistory[celestItem.name].push([x, y, z]);
        });
        return newHistory;
      });
      
      let lastDate = new Date(result[result.length - 1].timestamp);
      lastDate = new Date(lastDate.getTime() + passedTime);
      setDate(lastDate);
    } else {
      setLoaded(true);
      setDate(new Date(date.getTime() + passedTime));
    }
  }

  return (
    <div className="p-6 text-white space-y-6">
      <div className="flex items-center justify-between bg-zinc-900 p-4 rounded-xl shadow-inner w-full">
        
        {/* Bouton pause à gauche */}
        <div className="flex-1 flex justify-start">
          <button
            onClick={() => setPaused(!paused)}
            className="flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg bg-blue-600 hover:bg-blue-500 transition min-w-[120px]"
          >
            {paused ? (
              <>
                <span className="text-lg">▶</span> Start
              </>
            ) : (
              <>
                <span className="text-lg">⏸</span> Stop
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
