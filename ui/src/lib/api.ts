const API_BASE_URL = 'http://localhost:8080';

export async function ping(): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/ping`);
  if (!response.ok) throw new Error('Erreur lors du ping API');
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
  return response.json();
}
