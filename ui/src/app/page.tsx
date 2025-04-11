'use client';

import { useEffect, useState } from 'react';
import SolarSystem from '../components/SolarSystem';
import { ping } from '../lib/api';

export default function Home() {
  const [message, setMessage] = useState<string>('En attente du serveur...');

  useEffect(() => {
    ping()
      .then(setMessage)
      .catch((err) => setMessage(`Erreur : ${err.message}`));
  }, []);

  useEffect(() => {
    console.log(message);
  }, [message]);

  return (
    <main className="min-h-screen bg-gray-950">
      <SolarSystem />
    </main>
  );
}