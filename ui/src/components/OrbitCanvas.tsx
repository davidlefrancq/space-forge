'use client';

import React, { useEffect, useRef } from 'react';

interface Planet {
  name: string;
  position: [number, number, number];
  radius: number;
}

interface OrbitCanvasProps {
  planets: Planet[];
}

const OrbitCanvas: React.FC<OrbitCanvasProps> = ({ planets }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width = canvas.offsetWidth;
    const height = canvas.height = canvas.offsetHeight;

    ctx.clearRect(0, 0, width, height);
    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, width, height);

    const sunX = width / 2;
    const sunY = height / 2;

    // Dessiner le Soleil
    ctx.fillStyle = 'yellow';
    ctx.beginPath();
    ctx.arc(sunX, sunY, 8, 0, 2 * Math.PI);
    ctx.fill();

    // Calcul dynamique de l'échelle
    const maxDistance = Math.max(...planets.map(p =>
      Math.sqrt(p.position[0] ** 2 + p.position[1] ** 2)
    ));

    const padding = 40; // marge autour des orbites
    const scale = (Math.min(width, height) / 2 - padding) / maxDistance;

    planets.forEach(planet => {
      const x = sunX + planet.position[0] * scale;
      const y = sunY + planet.position[1] * scale;

      // Orbite simplifiée
      ctx.strokeStyle = 'rgba(255,255,255,0.1)';
      ctx.beginPath();
      const orbitRadius = Math.sqrt(planet.position[0] ** 2 + planet.position[1] ** 2) * scale;
      ctx.arc(sunX, sunY, orbitRadius, 0, 2 * Math.PI);
      ctx.stroke();

      // Planète
      ctx.fillStyle = 'white';
      ctx.beginPath();
      ctx.arc(x, y, 4, 0, 2 * Math.PI);
      ctx.fill();

      // Nom
      ctx.fillStyle = 'white';
      ctx.font = '12px Arial';
      ctx.fillText(planet.name, x + 6, y - 6);
    });

  }, [planets]);

  return (
    <div className="w-full h-[600px] bg-slate-950 rounded-xl overflow-hidden shadow-lg">
      <canvas ref={canvasRef} className="w-full h-full"></canvas>
    </div>
  );
};

export default OrbitCanvas;
