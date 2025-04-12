'use client';

import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { Texture } from 'three';
import { useTexture } from '@react-three/drei';

const POSITION_SCALE = 1e9;
const RADIUS_SCALE = 1e6;
const SUN_RADIUS_SCALE = 2e7;


interface Planet {
  name: string;
  position: [number, number, number];
  radius: number;
}

interface SolarSystem3DProps {
  planets: Planet[];
}

function SolarSystemScene({ planets }: { planets: Planet[] }) {

  const textures: {
    [key: string]: Texture;
  } = {
    Terre: useTexture('/textures/2k_earth_daymap.jpg'),
    Mars: useTexture('/textures/2k_mars.jpg'),
    Jupiter: useTexture('/textures/2k_jupiter.jpg'),
    Vénus: useTexture('/textures/2k_venus_atmosphere.jpg'),
    Mercure: useTexture('/textures/2k_mercury.jpg'),
    Saturne: useTexture('/textures/2k_saturn.jpg'),
    Uranus: useTexture('/textures/2k_uranus.jpg'),
    Neptune: useTexture('/textures/2k_neptune.jpg'),
    Soleil: useTexture('/textures/2k_sun.jpg'), // optionnel, car le Soleil est souvent emissive
  };

  const soleil = planets.find((planet) => planet.name === 'Soleil');
  if (!soleil) return null;

  return (
    <>
      {/* Lumière générale et directionnelle */}
      <ambientLight intensity={0.3} />
      <directionalLight
        position={[0, 0, 100]}
        intensity={1.5}
        color="yellow"
        castShadow
      />

      <OrbitControls enablePan enableZoom enableRotate />

      {/* Soleil */}
      <mesh position={[0, 0, 0]}>
        <sphereGeometry args={[soleil.radius / SUN_RADIUS_SCALE, 64, 64]} />
        <meshStandardMaterial
          map={textures[soleil.name] || undefined}
          emissive="darkorange"
          emissiveIntensity={1.5}
        />
      </mesh>

      {/* Planètes */}
      {planets.map((planet) => (
        planet.name !== 'Soleil' && <mesh
          key={planet.name}
          position={[
            planet.position[0] / POSITION_SCALE,
            planet.position[1] / POSITION_SCALE,
            planet.position[2] / POSITION_SCALE,
          ]}
        >
          <sphereGeometry args={[planet.radius / RADIUS_SCALE, 64, 64]} />
          <meshStandardMaterial
            map={textures[planet.name] || undefined}
          />
        </mesh>
      ))}
    </>
  );
}


export default function SolarSystem3D({ planets }: SolarSystem3DProps) {
  return (
    <div className="w-full h-[600px] rounded-xl overflow-hidden shadow-lg">
      <Canvas camera={{ position: [0, 0, 1000], near: 0.1, far: 1e7 }}>
        <SolarSystemScene planets={planets} />
      </Canvas>
    </div>
  );
}