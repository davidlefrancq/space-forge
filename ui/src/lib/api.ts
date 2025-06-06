const API_BASE_URL = 'http://localhost:8080';

export async function ping(): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/`);
  if (!response.ok) throw new Error('Erreur lors du ping de l\'API');
  return response.text();
}

export async function simulate(date: string) {
  const response = await fetch(`${API_BASE_URL}/simulate`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ date }),
  });

  if (!response.ok) throw new Error('Erreur lors de la simulation');
  return JSON.parse(await response.json());
}

export async function getSimulatedRange(from: string, to: string, step_seconds: number) {
  const response = await fetch(`${API_BASE_URL}/get_simulated_range`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ from, to, step_seconds }),
  });

  if (!response.ok) throw new Error('Erreur lors de la simulation');
  return JSON.parse(await response.json());
}