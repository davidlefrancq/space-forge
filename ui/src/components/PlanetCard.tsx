import React from 'react';

interface Planet {
  name: string;
  mass: number;
  radius: number;
  position: [number, number, number];
  velocity: [number, number, number];
}

interface PlanetCardProps {
  planet: Planet;
}

const PlanetCard: React.FC<PlanetCardProps> = ({ planet }) => {
  return (
    <div className="bg-gray-800 p-4 rounded-lg shadow-md text-white">
      <h2 className="text-xl font-semibold">{planet.name}</h2>
      <ul className="mt-2 text-sm">
        <li><strong>Masse :</strong> {planet.mass.toExponential(2)} kg</li>
        <li><strong>Rayon :</strong> {(planet.radius / 1000).toLocaleString()} km</li>
        <li>
          <strong>Position :</strong> [{planet.position.map(p => p.toExponential(2)).join(', ')}] m
        </li>
        <li>
          <strong>Vitesse :</strong> [{planet.velocity.map(v => v.toExponential(2)).join(', ')}] m/s
        </li>
      </ul>
    </div>
  );
};

export default PlanetCard;
