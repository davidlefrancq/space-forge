'use client';

import { Canvas } from '@react-three/fiber';
import { OrbitControls, useTexture } from '@react-three/drei';
import { Texture } from 'three';

const POSITION_SCALE = 1e9;

interface CelestItem {
  name: string;
  position: [number, number, number];
  radius: number;
}

interface SolarSystem3DProps {
  celestItems: CelestItem[];
  orbitHistory?: Record<string, [number, number, number][]>;
}

function SolarSystemScene({ celestItems, orbitHistory }: SolarSystem3DProps) {

  // Couleur simple par nom
  const colors: Record<string, string> = {
    Terre: 'blue',
    Mars: 'red',
    Mercure: 'gray',
    Venus: 'yellow',
    Jupiter: 'orange',
    Saturne: 'khaki',
    Uranus: 'cyan',
    Neptune: 'purple',
  };

  const textures: {
    [key: string]: Texture;
  } = {
    Terre: useTexture('/textures/2k_earth_daymap.jpg'),
    Mars: useTexture('/textures/2k_mars.jpg'),
    Jupiter: useTexture('/textures/2k_jupiter.jpg'),
    Venus: useTexture('/textures/2k_venus_atmosphere.jpg'),
    Mercure: useTexture('/textures/2k_mercury.jpg'),
    Saturne: useTexture('/textures/2k_saturn.jpg'),
    Uranus: useTexture('/textures/2k_uranus.jpg'),
    Neptune: useTexture('/textures/2k_neptune.jpg'),
    Soleil: useTexture('/textures/2k_sun.jpg'), // optionnel, car le Soleil est souvent emissive
  };

  const soleil = celestItems.find((celestItem) => celestItem.name === 'Soleil');
  if (!soleil) return null;

  function scaleRadius(
    radius: number,
    maxRadius: number,
    options?: { exponent?: number; scaleMax?: number; min?: number }
  ): number {
    const { exponent = 0.25, scaleMax = 16, min = 1.0 } = options || {};
    const normalized = radius / maxRadius;
    const scaled = Math.pow(normalized, exponent) * scaleMax;
    return Math.max(scaled, min);
  }

  const maxRadius = Math.max(...celestItems.map((item) => item.radius));

  return (
    <>
      {/* Lumière générale et directionnelle */}
      <ambientLight intensity={1} />
      <directionalLight
        position={[0, 0, 500]}
        intensity={0.5}
      />

      <OrbitControls enablePan enableZoom enableRotate />

      {/* Soleil */}
      <mesh position={[0, 0, 0]}>
        <sphereGeometry args={[scaleRadius(soleil.radius, maxRadius), 64, 64]} />
        <meshStandardMaterial
          map={textures[soleil.name] || undefined}
          emissive="darkorange"
          emissiveIntensity={1.5}
        />
      </mesh>

      {/* Planètes */}
      {celestItems.map((celestItem) =>
        celestItem.name !== 'Soleil' ? (
          <mesh
            key={celestItem.name}
            position={[
              celestItem.position[0] / POSITION_SCALE,
              celestItem.position[1] / POSITION_SCALE,
              celestItem.position[2] / POSITION_SCALE,
            ]}
          >
            <sphereGeometry args={[scaleRadius(celestItem.radius, maxRadius), 64, 64]} />
            <meshStandardMaterial map={textures[celestItem.name] || undefined} />
          </mesh>
        ) : null
      )}

      {/* Points des orbites */}
      {orbitHistory &&
        Object.entries(orbitHistory).flatMap(([name, path]) =>
          path.map(([x, y, z], index) => (
            <mesh
              key={`${name}-point-${index}`}
              position={[
                x / POSITION_SCALE,
                y / POSITION_SCALE,
                z / POSITION_SCALE,
              ]}
            >
              <sphereGeometry args={[1, 8, 8]} />
              <meshBasicMaterial color={colors[name] || 'white'} />
            </mesh>
          ))
        )}
    </>
  );
}


export default function SolarSystem3D({ celestItems, orbitHistory }: SolarSystem3DProps) {
  return (
    <div className="w-full h-[600px] rounded-xl overflow-hidden shadow-lg">
      <Canvas
        camera={{ position: [0, 0, 300], near: 0.1, far: 1e7 }}
        style={{ background: 'black' }}
      >
        <SolarSystemScene celestItems={celestItems} orbitHistory={orbitHistory} />
      </Canvas>
    </div>
  );
}